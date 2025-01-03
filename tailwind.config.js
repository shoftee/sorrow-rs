/** @type {import('tailwindcss').Config} */
module.exports = {
    content: {
        files: ["*.html", "./crates/ui/src/**/*.rs"],
        transform: {
            rs: (content) => content.replace(/(?:^|\s)class:/g, ' '),
        },
    },
    theme: {
        extend: {},
    },
    plugins: [],
}