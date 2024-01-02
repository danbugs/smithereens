const express = require('express');
const fs = require('fs');
const path = require('path');

const app = express();

// Middleware to handle JSON payloads
app.use(express.json({ limit: '50mb' }));

// Endpoint for uploading base64 encoded image
app.post('/upload', (req, res) => {
    const base64Image = req.body.image;
    const filename = 'image_' + Date.now() + '.png'; // or any other extension
    const filePath = path.join(__dirname, 'uploads', filename);

    // Decode the base64 string to binary data
    const binaryData = Buffer.from(base64Image, 'base64');
    
    // Save the binary data as an image file
    fs.writeFile(filePath, binaryData, (err) => {
        if (err) {
            console.error(err);
            return res.status(500).send('Error saving the image');
        }
        res.json({ message: 'Image uploaded successfully', filePath: filePath });
    });
});

// Serve images directly
app.use('/images', express.static(path.join(__dirname, 'uploads')));

// Endpoint to serve the HTML with Twitter Card metadata
app.get('/image/:imageName', (req, res) => {
    const imageName = req.params.imageName;
    const imagePath = path.join(__dirname, 'uploads', imageName);

    // Check if image exists
    if (!fs.existsSync(imagePath)) {
        return res.status(404).send('Image not found');
    }

    // Serve an HTML page with Twitter Card metadata
    res.send(`
        <html>
            <head>
                <meta name="twitter:card" content="summary_large_image">
                <meta name="twitter:image" content="${'http://smithe.pictures/images/' + imageName}">
                <meta name="twitter:title" content="Tourney Result">
            </head>
            <body>
                <img src="${'/images/' + imageName}" alt="Image">
            </body>
        </html>
    `);
});

const PORT = process.env.PORT || 3000;
app.listen(PORT, () => {
    console.log(`Server is running on port ${PORT}`);
});
