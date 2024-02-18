const express = require('express');
const fs = require('fs');
const path = require('path');
const cors = require('cors');
const fetch = require('node-fetch');
const bodyParser = require('body-parser');
const rateLimit = require('express-rate-limit');

const app = express();

// Enable CORS for all requests

// CORS configuration to allow only smithe.net
const corsOptions = {
    origin: ['http://smithe.net', 'https://smithe.net'],
    optionsSuccessStatus: 200 // For legacy browser support
  };

app.use(cors(corsOptions));
app.use(bodyParser.json());

const RECAPTCHA_THRESHOLD = 0.5; // Set the threshold as per your requirement

// Check if RECAPTCHA_SECRET_KEY is set
if (!process.env.RECAPTCHA_SECRET_KEY) {
    console.error('RECAPTCHA_SECRET_KEY is not set. Exiting.');
    process.exit(1);
}

// Middleware to handle JSON payloads
app.use(express.json({ limit: '50mb' }));

// Apply rate limiting
const limiter = rateLimit({
    windowMs: 15 * 60 * 1000, // 15 minutes
    max: 100, // limit each IP to 100 requests per windowMs
    handler: function (req, res /*, next */) {
        console.error(`Rate limit exceeded for IP: ${req.ip}`);
        res.status(429).send('Too many requests, please try again later.');
    }
});
app.use(limiter);

// File type validation function
const isValidImageType = (dataString) => {
    const matches = dataString.match(/^data:([A-Za-z-+\/]+);base64,/);
    if (!matches) return false;
    const mimeType = matches[1].toLowerCase();
    return mimeType.startsWith('image/');
};

// Endpoint for uploading base64 encoded image
app.post('/upload', (req, res) => {
    var base64Image = req.body.image;

    // Validate image type
    if (!isValidImageType(base64Image)) {
        return res.status(400).send('Invalid image type');
    }

    // Remove the prefix "data:image/png;base64,"
    const prefix = /^data:image\/\w+;base64,/;
    base64Image = base64Image.replace(prefix, "");

    const filename = 'image_' + Date.now() + '.png'; // or any other extension
    const filePath = path.join(__dirname, 'uploads', path.basename(filename));

    // Decode the base64 string to binary data
    const binaryData = Buffer.from(base64Image, 'base64');

    // Save the binary data as an image file
    fs.writeFile(filePath, binaryData, (err) => {
        if (err) {
            console.error('Internal Error: Unable to save image.');
            return res.status(500).send('Error saving the image');
        }
        res.json({ message: 'Image uploaded successfully', filename: filename });
    });
});

// Serve images directly
app.use('/images', express.static(path.join(__dirname, 'uploads')));

// Check captcha token
app.post('/check-captcha', (req, res) => {
    const captchaToken = req.body.token;
    const secretKey = process.env.RECAPTCHA_SECRET_KEY; // Make sure this environment variable is set
    const verificationUrl = `https://www.google.com/recaptcha/api/siteverify`;

    // Make a request to verify the captcha token
    fetch(verificationUrl, {
        method: 'POST',
        headers: {
            'Content-Type': 'application/x-www-form-urlencoded',
        },
        body: `secret=${secretKey}&response=${captchaToken}`
    })
    .then(response => response.json())
    .then(data => {
        if (data.success && data.score >= RECAPTCHA_THRESHOLD) {
            res.json({ message: 'Captcha verification successful', score: data.score });
        } else {
            res.status(400).json({ message: 'Captcha verification failed', score: data.score });
        }
    })
    .catch(error => {
        console.error('Error verifying captcha:', error);
        res.status(500).json({ message: 'Error verifying captcha' });
    });
});

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
    console.log(`Server is running on port ${PORT}`);
});
