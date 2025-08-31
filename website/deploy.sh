#!/bin/bash

# Deployment script for Leptos Motion website
set -e

echo "🚀 Deploying Leptos Motion website..."

# Build the website
./build.sh

# Check if we're in a git repository
if [ -d ".git" ]; then
    echo "📦 Git repository detected"
    
    # Check if we're on the main branch
    CURRENT_BRANCH=$(git branch --show-current)
    if [ "$CURRENT_BRANCH" = "main" ] || [ "$CURRENT_BRANCH" = "master" ]; then
        echo "✅ On main branch ($CURRENT_BRANCH)"
        
        # Check if there are uncommitted changes
        if [ -n "$(git status --porcelain)" ]; then
            echo "⚠️  Warning: There are uncommitted changes"
            echo "   Consider committing your changes before deploying"
        fi
        
        echo "📤 Ready for deployment!"
        echo ""
        echo "Deployment options:"
        echo "1. GitHub Pages: Push to main branch and enable Pages in repository settings"
        echo "2. Netlify: Drag and drop the 'dist' folder to netlify.com"
        echo "3. Vercel: Connect your repository to vercel.com"
        echo "4. Manual: Upload 'dist' folder to your hosting provider"
        echo ""
        echo "📁 Built files are in: dist/"
    else
        echo "⚠️  Not on main branch (current: $CURRENT_BRANCH)"
        echo "   Switch to main branch for deployment"
    fi
else
    echo "📦 No git repository detected"
    echo "📁 Built files are in: dist/"
    echo ""
    echo "Deployment options:"
    echo "1. Netlify: Drag and drop the 'dist' folder to netlify.com"
    echo "2. Vercel: Upload the 'dist' folder to vercel.com"
    echo "3. Manual: Upload 'dist' folder to your hosting provider"
fi

echo ""
echo "✅ Deployment script completed!"
