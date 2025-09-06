# Leptos Motion Website

A static website showcasing the Leptos Motion animation library.

## Features

- **Interactive Examples**: Demonstrates key animation features
- **Installation Guide**: Shows how to add leptos-motion to your project
- **Responsive Design**: Works on desktop and mobile devices
- **Modern UI**: Clean, professional design with smooth animations

## Development

### Prerequisites

- A modern web browser
- Python 3 (for local serving) or Node.js (for serve)

### Local Development

1. **Build the website:**

   ```bash
   ./build.sh
   ```

2. **Serve locally:**

   ```bash
   # Using Python
   cd dist && python3 -m http.server 8000

   # Or using Node.js serve
   cd dist && npx serve .
   ```

3. **Open in browser:**
   - Python: http://localhost:8000
   - Serve: http://localhost:3000

### File Structure

```
website/
├── index.html          # Main website file
├── build.sh           # Build script
└── README.md          # This file
```

## Customization

The website is built with vanilla HTML, CSS, and JavaScript. To customize:

- **Styling**: Edit the `<style>` section in `index.html`
- **Content**: Modify the HTML structure
- **Interactions**: Update the JavaScript in the `<script>` section

## Deployment

The website can be deployed to any static hosting service:

- **GitHub Pages**: Push to a repository and enable Pages
- **Netlify**: Drag and drop the `dist` folder
- **Vercel**: Connect your repository
- **AWS S3**: Upload the `dist` folder

## Contributing

To contribute to the website:

1. Make your changes to `index.html`
2. Test locally using the build script
3. Submit a pull request

## License

Same as the main Leptos Motion project.
