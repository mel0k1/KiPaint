import { browserVersion, operatingSystem } from "/src/utility-functions/platform";
import { stripIndents } from "/src/utility-functions/strip-indents";

export function crashReportUrl(panicDetails: string): string {
	const url = new URL("https://github.com/mel0k1/KiPaint/issues/new");

	const buildUrl = (includeCrashReport: boolean) => {
		let body = stripIndents`
			**Describe the Crash**
			Explain clearly what you were doing when the crash occurred.

			**Steps To Reproduce**
			Describe precisely how the crash occurred, step by step, starting with a new editor window.
			1. Open KiPaint and reproduce the issue.
			2.
			3.
			4.
			5.

			**Additional Details**
			Provide any further information or context that you think would be helpful in fixing the issue. Screenshots or video can be linked or attached to this issue.

			**Browser and OS**
			${browserVersion()}, ${operatingSystem()}

			**Stack Trace**
			Copied from the crash dialog in KiPaint:
		`;

		const manualCopyStackTraceNotice = stripIndents`
			Before submitting this bug, REPLACE THIS WITH THE LOG. Return to the editor and click "Copy Error Log" in the crash dialog and paste it in place of this text.
		`;

		body += "\n\n```\n";
		body += includeCrashReport ? panicDetails.trimEnd() : manualCopyStackTraceNotice;
		body += "\n```";

		const fields = {
			title: "[Crash Report] ",
			body,
			labels: ["Crash"].join(","),
			projects: [].join(","),
			milestone: "",
			assignee: "",
			template: "",
		};

		Object.entries(fields).forEach(([field, value]) => {
			if (value) url.searchParams.set(field, value);
		});

		return String(url);
	};

	let urlString = buildUrl(true);
	if (urlString.length >= 8192) {
		// Fall back to a shorter version if it exceeds GitHub limits of 8192 total characters
		urlString = buildUrl(false);
	}
	return urlString;
}
