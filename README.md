# Imoost

Imoost is an open-source, self-hosted image optimization service built for Next.js in Rust. It leverages [imgproxy](https://github.com/imgproxy/imgproxy) to dynamically transform, resize, and compress your images for faster load times and better performance—all while giving you full control over your image infrastructure. ✨

---

## Features

- 🔐 **Signing of URLs** (optional)
- 🔗 **Support for multiple domains**
- 🐳 **Docker support**
- 🏠 **Self-hosted**
- 💻 **Open-source**

---

## Table of Contents

1. [Getting Started](#getting-started)  
   1.1 [Installation](#installation)  
   1.2 [Configuration](#configuration)  
2. [Usage](#usage)  
   2.1 [Running with Docker (Coolify)](#running-with-docker-coolify)  
   2.2 [Running with Docker Compose (Standalone)](#running-with-docker-compose-standalone)  
3. [Next.js Integration](#nextjs-integration)  
   3.1 [Creating a Custom Loader](#creating-a-custom-loader)  
   3.2 [Updating `next.config.js`](#updating-nextconfigjs)  
4. [Environment Variables](#environment-variables)  
5. [License & Credits](#license--credits)

---

## Getting Started

### Installation

To install Imoost with Coolify, follow these steps:

1. **Create a new resource** in your Coolify project.
2. **Choose** the **"Docker Compose Empty"** template.
3. **Copy and paste** the following Docker Compose configuration into the editor:
    
    ```yaml
    services:
      imoost:
        image: 'ghcr.io/kyomawa/imoost/imoost-imoost:latest'
        container_name: imoost
        ports:
          - '8001:8000'
        environment:
          - 'IMGPROXY_URL=${IMGPROXY_URL}'
          - 'ALLOWED_DOMAINS=${ALLOWED_DOMAINS}'
          - 'IMGPROXY_KEY=${IMGPROXY_KEY}'
          - 'IMGPROXY_SALT=${IMGPROXY_SALT}'
        depends_on:
          imgproxy:
            condition: service_healthy
        healthcheck:
          test:
            - CMD
            - curl
            - '-f'
            - 'http://localhost:8000/health'
          interval: 2s
          timeout: 10s
          retries: 5
        networks:
          - coolify
    
      imgproxy:
        image: 'darthsim/imgproxy:latest'
        container_name: imgproxy
        environment:
          - 'IMGPROXY_AUTO_WEBP=${IMGPROXY_AUTO_WEBP}'
          - 'IMGPROXY_AUTO_AVIF=${IMGPROXY_AUTO_AVIF}'
          - 'IMGPROXY_JPEG_PROGRESSIVE=${IMGPROXY_JPEG_PROGRESSIVE}'
          - 'IMGPROXY_USE_ETAG=${IMGPROXY_USE_ETAG}'
          - 'IMGPROXY_KEY=${IMGPROXY_KEY}'
          - 'IMGPROXY_SALT=${IMGPROXY_SALT}'
          - 'ALLOWED_DOMAINS=${ALLOWED_DOMAINS}'
        healthcheck:
          test:
            - CMD
            - imgproxy
            - health
          interval: 2s
          timeout: 10s
          retries: 5
        ports:
          - '8081:8080'
        networks:
          - coolify
    
    networks:
      coolify:
        external: true
    
    ```
    
4. **Deploy your resource.**
    
    After validating the configuration, two new services will appear in Coolify: **imoost** and **imgproxy**.
    
    ⚠️ **Important:**
    
    Go to the settings of the **imoost** service in Coolify and set the domain name for your service (for example, `https://image.mydomain.com`). This ensures that Imoost uses the correct domain for URL generation and integration.
    

---

## Configuration

Imoost relies on environment variables to configure how it communicates with imgproxy and how it signs URLs (if needed). You must configure these variables in your deployment settings. Refer to the [Environment Variables](https://www.notion.so/1ab599302505803da3a3ec4f48a30ff3?pvs=21) section below for details.

---

## Usage

### Running with Docker (Coolify)

If you are deploying via Coolify using the Docker Compose configuration provided above, Coolify will handle the networking and service management for you. Once deployed, ensure that you set the proper domain name for the imoost service in its settings (e.g., `https://image.mydomain.com`).

### Running with Docker Compose (Standalone)

If you prefer to run Imoost locally or without Coolify, use the following Docker Compose configuration without the external network settings:

```yaml
services:
  imoost:
    image: 'ghcr.io/kyomawa/imoost/imoost-imoost:latest'
    container_name: imoost
    ports:
      - "8000:8000"
    environment:
      - IMGPROXY_URL=${IMGPROXY_URL}
      - ALLOWED_DOMAINS=${ALLOWED_DOMAINS}
      - IMGPROXY_KEY=${IMGPROXY_KEY}
      - IMGPROXY_SALT=${IMGPROXY_SALT}
    depends_on:
      imgproxy:
        condition: service_healthy
    healthcheck:
      test: ["CMD", "curl", "-f", "http://localhost:8000/health"]
      interval: 2s
      timeout: 10s
      retries: 5

  imgproxy:
    image: darthsim/imgproxy:latest
    container_name: imgproxy
    environment:
      - IMGPROXY_AUTO_WEBP=${IMGPROXY_AUTO_WEBP}
      - IMGPROXY_AUTO_AVIF=${IMGPROXY_AUTO_AVIF}
      - IMGPROXY_JPEG_PROGRESSIVE=${IMGPROXY_JPEG_PROGRESSIVE}
      - IMGPROXY_USE_ETAG=${IMGPROXY_USE_ETAG}
      - IMGPROXY_KEY=${IMGPROXY_KEY}
      - IMGPROXY_SALT=${IMGPROXY_SALT}
      - ALLOWED_DOMAINS=${ALLOWED_DOMAINS}
    healthcheck:
      test: ["CMD", "imgproxy", "health"]
      interval: 2s
      timeout: 10s
      retries: 5
    ports:
      - "8081:8080"

```

Simply run:

```docker
docker-compose up --build
```

- **Imoost** will be available at: [http://localhost:8000](http://localhost:8000/)
- **Health check endpoint:** http://localhost:8000/health
- **imgproxy** will be available at: [http://localhost:8080](http://localhost:8080/)

---

## Next.js Integration

Imoost is designed to replace the default Next.js Image Optimization API. Instead of using `/api` routes from Next.js, you’ll use your Imoost endpoint to optimize images. Below is an example of how to integrate it using a **custom loader**.

### Creating a Custom Loader

1. In your Next.js project, **create a file** named `imagesLoader.ts` at the root (or another convenient location):
    
    ```tsx
    "use client";
    
    // =======================================================================================================
    
    import { ImageProps } from "next/image";
    
    // =======================================================================================================
    
    export type imageLoaderProps = {
      src: string;
      width: ImageProps["width"];
      quality: ImageProps["quality"];
    };
    
    // =======================================================================================================
    
    export default function imageLoader({ src, width, quality }: imageLoaderProps) {
      const imageIsLocal = !src.startsWith("http");
    
      const query = new URLSearchParams();
      if (width) {
        query.set("width", width.toString());
      }
      if (quality) {
        query.set("quality", quality.toString());
      }
    
      // Replace <image-optimization-domain> with your Imoost domain (e.g., 'https://image.mydomain.com')
      const imageOptimizationApi = "<image-optimization-domain>";
    
      // Replace <your-nextjs-app-domain> with your Next.js app domain (e.g., 'https://mynextjsapp.com')
      const baseUrl = "<your-nextjs-app-domain>";
    
      // Build the full source for local images
      const fullSrc = `${baseUrl}${src}`;
    
      if (imageIsLocal && process.env.NODE_ENV === "development") {
        return src;
      }
    
      if (imageIsLocal) {
        return `${imageOptimizationApi}/image/${fullSrc}?${query.toString()}`;
      }
    
      return `${imageOptimizationApi}/image/${src}?${query.toString()}`;
    }
    
    // =======================================================================================================
    
    ```
    

### Updating `next.config.js`

1. In your `next.config.js` or `next.config.ts`, **configure Next.js** to use your custom loader:
    
    ```ts
    import type { NextConfig } from "next";
    
    const nextConfig: NextConfig = {
      images: {
        loader: "custom",
        loaderFile: "./imagesLoader.ts", // or wherever you placed it
      },
    };
    
    export default nextConfig;
    
    ```
    
2. **Restart your Next.js server** so the configuration changes take effect. You should now be able to use the `<Image>` component from Next.js, and it will route image optimization through Imoost.

---

## Environment Variables

Below is a summary of the environment variables that Imoost uses:

| Variable | Description | Default |
| --- | --- | --- |
| **IMGPROXY_URL** | The URL for your imgproxy server. Example: `http://imgproxy:8080` | `http://imgproxy:8080` |
| **ALLOWED_DOMAINS** | A comma-separated list of allowed domains. Use `*` to allow all. Example: `example.com,another.com` | `*` |
| **IMGPROXY_KEY** | Hex-encoded key for signing URLs (optional). If empty, no signing is performed. | (none) |
| **IMGPROXY_SALT** | Hex-encoded salt for signing URLs (optional). If empty, no signing is performed. | (none) |
| **IMGPROXY_AUTO_WEBP** | `true/false`. Auto-detects if the client supports WebP. | `true` |
| **IMGPROXY_AUTO_AVIF** | `true/false`. Auto-detects if the client supports AVIF. | `true` |
| **IMGPROXY_JPEG_PROGRESSIVE** | `true/false`. Generates progressive JPEGs if `true`. | `true` |
| **IMGPROXY_USE_ETAG** | `true/false`. Includes ETag headers if `true`. | `true` |

> Note
> 
> 
> If you want to sign your URLs, you **must** provide both `IMGPROXY_KEY` and `IMGPROXY_SALT`. These should be hex-encoded strings.
> 

---

## License & Credits

Imoost is open-source software licensed under the MIT license (if provided in the repo).

**Credits:**

This project was heavily inspired by the [next-image-transformation](https://github.com/coollabsio/next-image-transformation) project by Coollabs. Special thanks to the original authors for their work!

---

**Happy optimizing!** 🚀