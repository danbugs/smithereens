const express = require('express');
const fs = require('fs');
const path = require('path');
const cors = require('cors');
const bodyParser = require('body-parser');
const rateLimit = require('express-rate-limit');

const app = express();

// Enable CORS for all requests
const corsOptions = {
    origin: ['http://smithe.net', 'https://smithe.net'],
    optionsSuccessStatus: 200
};
app.use(cors(corsOptions));
app.use(bodyParser.json());

// Apply global rate limiting
const limiter = rateLimit({
    windowMs: 15 * 60 * 1000, // 15 minutes
    max: 100, // limit each IP to 100 requests per windowMs
    handler: function (req, res) {
        console.error(`Rate limit exceeded for IP: ${req.ip}`);
        res.status(429).send('Too many requests, please try again later.');
    }
});
app.use(limiter);

// Additional strict rate limit for image uploads
const uploadLimiter = rateLimit({
    windowMs: 10 * 60 * 1000, // 10 minutes
    max: 10, // limit each IP to 10 uploads per windowMs
    handler: function (req, res) {
        console.error(`Upload rate limit exceeded for IP: ${req.ip}`);
        res.status(429).send('Too many uploads, please try again later.');
    }
});

// Simple IP-based request tracking
const requestTracker = {};
const MAX_UPLOADS_PER_HOUR = 20;

const trackRequest = (ip) => {
    const now = Date.now();
    requestTracker[ip] = requestTracker[ip] || [];
    requestTracker[ip].push(now);

    // Remove requests older than 1 hour
    requestTracker[ip] = requestTracker[ip].filter(timestamp => now - timestamp < 60 * 60 * 1000);

    return requestTracker[ip].length > MAX_UPLOADS_PER_HOUR;
};

// File type validation function
const isValidImageType = (dataString) => {
    const matches = dataString.match(/^data:([A-Za-z-+\/]+);base64,/);
    if (!matches) return false;
    const mimeType = matches[1].toLowerCase();
    return mimeType.startsWith('image/');
};

// Endpoint for uploading base64 encoded images
app.post('/upload', uploadLimiter, (req, res) => {
    const base64Image = req.body.image;
    const honeypot = req.body.honeypot; // Hidden field to trap bots

    // Detect bot submissions
    if (honeypot) {
        console.warn(`Honeypot triggered by IP: ${req.ip}`);
        return res.status(400).send('Spam detected');
    }

    // Enforce additional upload limits
    if (trackRequest(req.ip)) {
        console.warn(`Upload abuse detected from IP: ${req.ip}`);
        return res.status(429).send('Upload limit exceeded');
    }

    // Validate image type
    if (!isValidImageType(base64Image)) {
        return res.status(400).send('Invalid image type');
    }

    // Remove the prefix "data:image/png;base64,"
    const prefix = /^data:image\/\w+;base64,/;
    const cleanImage = base64Image.replace(prefix, "");

    const filename = 'image_' + Date.now() + '.png';
    const filePath = path.join(__dirname, 'uploads', path.basename(filename));

    // Decode and save image
    fs.writeFile(filePath, Buffer.from(cleanImage, 'base64'), (err) => {
        if (err) {
            console.error('Error saving image:', err);
            return res.status(500).send('Error saving the image');
        }
        res.json({ message: 'Image uploaded successfully', filename });
    });
});

// Serve images
app.use('/images', express.static(path.join(__dirname, 'uploads')));

// Fake CAPTCHA verification for compatibility
app.post('/check-captcha', (req, res) => {
    res.json({ message: 'Captcha verification successful', score: 1.0 });
});

// Serve metadata for Twitter Card images
app.get('/image/:imageName', (req, res) => {
    const imageName = req.params.imageName;
    const imagePath = path.join(__dirname, 'uploads', imageName);

    if (!fs.existsSync(imagePath)) {
        return res.status(404).send('Image not found');
    }

    res.send(`
        <html>
            <head>
                <meta name="twitter:card" content="summary_large_image">
                <meta name="twitter:image" content="${'https://smithe.pictures/images/' + imageName}">
                <meta name="twitter:title" content="smithe.net">
            </head>
            <body>
                <img src="${'/images/' + imageName}" alt="Image">
            </body>
        </html>
    `);
});

app.get('/', (req, res) => {
    res.send('Hello, World!');
});

const PORT = process.env.PORT || 3000;
app.listen(PORT, () => {
    console.log(`Server running on port ${PORT}`);
});
