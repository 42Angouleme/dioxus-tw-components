/** @type {import('tailwindcss').Config} */
module.exports = {
	mode: "all",
	content: [
		// include all rust, html and css files in the src directory
		"../dioxus-tw-components-style.css",
	],
	theme: {
		extend: {
			colors: {
				"background": 'var(--background)',
				"foreground": 'var(--foreground)',
				"primary": 'var(--primary)',
				"primary-foreground": 'var(--primary-foreground)',
				"secondary": 'var(--secondary))',
				"secondary-foreground": 'var(--secondary-foreground)',
				"accent": 'var(--accent)',
				"accent-foreground": 'var(--accent-foreground)',
				"muted": 'var(--muted)',
				"muted-foreground": 'var(--muted-foreground)',
				"destructive": 'var(--destructive)',
				"destructive-foreground": 'var(--destructive-foreground)',
				"success": 'var(--success)',
				"success-foreground": 'var(--success-foreground)',
				"border": 'var(--border)',
				"input": 'var(--input)',
				"ring": 'var(--ring)',
			},
			borderRadius: {
				"global-radius": 'var(--radius)',
			},
			boxShadow: {
				"global-shadow": 'var(--shadow)',
			},
			keyframes: {
				"shimmer" : {
					"100%" : {
						"transform" : "translateX(200%)",
					}
				}
			},
			fontSize: {
			},
		},
	},
	plugins: [],
}
