<script lang="ts">
	import { getContext } from "svelte";
	import LayoutCol from "/src/components/layout/LayoutCol.svelte";
	import TextLabel from "/src/components/widgets/labels/TextLabel.svelte";
	import IconButton from "/src/components/widgets/buttons/IconButton.svelte";
	import { createSRgba8FromHsva, isSRgba8, sRgba8ToHSV, sRgba8ToRgbaCSS } from "/src/utility-functions/colors";
	import type { DocumentStore } from "/src/stores/document";
	import type { EditorWrapper, SRGBA8 } from "/wrapper/pkg/graphite_wasm_wrapper";

	const editor = getContext<EditorWrapper>("editor");
	const document = getContext<DocumentStore>("document");

	type Target = "primary" | "secondary";

	let target: Target = "primary";
	let hue = 0;
	let saturation = 0;
	let value = 1;
	let alpha = 1;
	let dragging = false;
	let svArea: HTMLDivElement | undefined;
	let hueArea: HTMLDivElement | undefined;

	// Mirror the working colors broadcast by the backend so the panel stays in sync with tool options and the shelf
	$: syncFromLayout($document.workingColorsLayout);

	function syncFromLayout(layout: unknown) {
		if (dragging) return;
		const colors = findWorkingColors(layout);
		if (!colors) return;
		const active = colors[target];
		if (!isSRgba8(active)) return;
		const hsv = sRgba8ToHSV(active);
		hue = hsv.h;
		saturation = hsv.s;
		value = hsv.v;
		alpha = active.alpha / 255;
	}

	function findWorkingColors(node: unknown): { primary?: SRGBA8; secondary?: SRGBA8 } | undefined {
		if (Array.isArray(node)) {
			for (const child of node) {
				const found = findWorkingColors(child);
				if (found) return found;
			}
			return undefined;
		}
		if (!node || typeof node !== "object") return undefined;
		const record = node as Record<string, unknown>;
		if (isSRgba8(record.primary) && isSRgba8(record.secondary)) return record as { primary: SRGBA8; secondary: SRGBA8 };
		for (const child of Object.values(record)) {
			if (!child || typeof child !== "object") continue;
			const found = findWorkingColors(child);
			if (found) return found;
		}
		return undefined;
	}

	function activeColor(): SRGBA8 {
		return createSRgba8FromHsva(hue, saturation, value, alpha);
	}

	function inactiveColor(colors: { primary?: SRGBA8; secondary?: SRGBA8 } | undefined): SRGBA8 {
		const fallback = createSRgba8FromHsva(hue, saturation, value, alpha);
		if (!colors) return fallback;
		const other = target === "primary" ? colors.secondary : colors.primary;
		return isSRgba8(other) ? other : fallback;
	}

	function applyColor() {
		const color = activeColor();
		if (target === "primary") editor.updatePrimaryColor(color);
		else editor.updateSecondaryColor(color);
	}

	function swapColors() {
		const colors = findWorkingColors($document.workingColorsLayout);
		const primary = colors?.primary;
		const secondary = colors?.secondary;
		if (isSRgba8(primary) && isSRgba8(secondary)) {
			editor.updateSecondaryColor(primary);
			editor.updatePrimaryColor(secondary);
			target = target === "primary" ? "secondary" : "primary";
		}
	}

	function pointerToFraction(e: PointerEvent, area: HTMLDivElement | undefined): { x: number; y: number } | undefined {
		if (!area) return undefined;
		const bounds = area.getBoundingClientRect();
		const x = (e.clientX - bounds.left) / bounds.width;
		const y = (e.clientY - bounds.top) / bounds.height;
		return { x: Math.min(Math.max(x, 0), 1), y: Math.min(Math.max(y, 0), 1) };
	}

	function svPointerDown(e: PointerEvent) {
		dragging = true;
		(e.target as HTMLElement).setPointerCapture(e.pointerId);
		svPointerMove(e);
	}

	function svPointerMove(e: PointerEvent) {
		if (!dragging) return;
		const point = pointerToFraction(e, svArea);
		if (!point) return;
		saturation = point.x;
		value = 1 - point.y;
		applyColor();
	}

	function svPointerUp(e: PointerEvent) {
		dragging = false;
		(e.target as HTMLElement).releasePointerCapture?.(e.pointerId);
	}

	function huePointerDown(e: PointerEvent) {
		dragging = true;
		(e.target as HTMLElement).setPointerCapture(e.pointerId);
		huePointerMove(e);
	}

	function huePointerMove(e: PointerEvent) {
		if (!dragging) return;
		const point = pointerToFraction(e, hueArea);
		if (!point) return;
		hue = point.x;
		applyColor();
	}

	function huePointerUp(e: PointerEvent) {
		dragging = false;
		(e.target as HTMLElement).releasePointerCapture?.(e.pointerId);
	}

	function setTarget(newTarget: Target) {
		target = newTarget;
		syncFromLayout($document.workingColorsLayout);
	}

	$: svBackground = `linear-gradient(to top, #000, transparent), linear-gradient(to right, #fff, hsl(${hue * 360}, 100%, 50%))`;
	$: hueBackground = `linear-gradient(to right, hsl(0,100%,50%), hsl(60,100%,50%), hsl(120,100%,50%), hsl(180,100%,50%), hsl(240,100%,50%), hsl(300,100%,50%), hsl(360,100%,50%))`;
	$: colorsFromLayout = findWorkingColors($document.workingColorsLayout);
	$: primaryCSS = colorsFromLayout?.primary ? sRgba8ToRgbaCSS(colorsFromLayout.primary) : "transparent";
	$: secondaryCSS = colorsFromLayout?.secondary ? sRgba8ToRgbaCSS(colorsFromLayout.secondary) : "transparent";
</script>

<LayoutCol class="colors-panel">
	<div
		class="sv-area"
		bind:this={svArea}
		style:background={svBackground}
		on:pointerdown={svPointerDown}
		on:pointermove={svPointerMove}
		on:pointerup={svPointerUp}
		on:pointercancel={svPointerUp}
		role="slider"
		aria-label="Saturation and value"
	>
		<div class="thumb" style:left={`${saturation * 100}%`} style:top={`${(1 - value) * 100}%`} style:background={sRgba8ToRgbaCSS(activeColor())}></div>
	</div>
	<div
		class="hue-area"
		bind:this={hueArea}
		style:background={hueBackground}
		on:pointerdown={huePointerDown}
		on:pointermove={huePointerMove}
		on:pointerup={huePointerUp}
		on:pointercancel={huePointerUp}
		role="slider"
		aria-label="Hue"
	>
		<div class="thumb hue-thumb" style:left={`${hue * 100}%`}></div>
	</div>
	<LayoutCol class="targets">
		<button class="target" class:active={target === "primary"} on:click={() => setTarget("primary")}>
			<span class="swatch" style:background={primaryCSS}></span>
			<TextLabel>Primary</TextLabel>
		</button>
		<button class="target" class:active={target === "secondary"} on:click={() => setTarget("secondary")}>
			<span class="swatch" style:background={secondaryCSS}></span>
			<TextLabel>Secondary</TextLabel>
		</button>
		<IconButton icon="SwapHorizontal" size={24} tooltipLabel="Swap colors" tooltipDescription="Exchange the primary and secondary working colors." action={swapColors} />
	</LayoutCol>
</LayoutCol>

<style lang="scss">
	.colors-panel {
		padding: 8px;
		gap: 8px;
		overflow-y: auto;

		.sv-area {
			position: relative;
			width: 100%;
			aspect-ratio: 16 / 10;
			border-radius: 2px;
			cursor: crosshair;
			touch-action: none;
		}

		.hue-area {
			position: relative;
			width: 100%;
			height: 14px;
			border-radius: 7px;
			cursor: ew-resize;
			touch-action: none;
		}

		.thumb {
			position: absolute;
			width: 14px;
			height: 14px;
			border-radius: 50%;
			transform: translate(-50%, -50%);
			border: 2px solid #fff;
			box-shadow: 0 0 2px rgba(0, 0, 0, 0.8);
			pointer-events: none;
			box-sizing: border-box;
		}

		.hue-thumb {
			top: 50%;
		}

		.targets {
			gap: 4px;

			.target {
				display: flex;
				align-items: center;
				gap: 8px;
				padding: 4px 8px;
				border: none;
				border-radius: 2px;
				background: transparent;
				cursor: pointer;
				text-align: left;

				&:hover {
					background: var(--color-4-mildgray);
				}

				&.active {
					background: var(--color-5-dullgray);
				}

				.swatch {
					width: 16px;
					height: 16px;
					border-radius: 50%;
					border: 1px solid var(--color-6-lowergray);
					box-sizing: border-box;
				}
			}
		}
	}
</style>
