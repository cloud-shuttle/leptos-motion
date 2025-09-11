#!/bin/bash

# Open the WebGL demo in the default browser
echo "🎮 Opening WebGL Advanced Features Demo..."

# Get the absolute path to the demo file
DEMO_PATH="$(cd "$(dirname "$0")" && pwd)/demo.html"

echo "📁 Demo location: $DEMO_PATH"

# Open in default browser
if command -v open &> /dev/null; then
    # macOS
    open "$DEMO_PATH"
elif command -v xdg-open &> /dev/null; then
    # Linux
    xdg-open "$DEMO_PATH"
elif command -v start &> /dev/null; then
    # Windows
    start "$DEMO_PATH"
else
    echo "❌ Could not find a way to open the browser automatically."
    echo "Please open this file manually in your browser:"
    echo "$DEMO_PATH"
fi

echo "✅ Demo should now be opening in your browser!"
echo ""
echo "🎯 What you'll see:"
echo "   • Interactive 3D scene with rotating cube"
echo "   • Real-time post-processing effects"
echo "   • Physics simulation with bouncing sphere"
echo "   • Live controls for all parameters"
echo "   • Performance monitoring"
echo ""
echo "🎮 Try adjusting the sliders to see real-time effects!"
