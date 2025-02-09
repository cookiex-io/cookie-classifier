# Cookie Classifier Microservice 

## Overview
This microservice provides an API for classifying cookies. It includes two main endpoints:
1. **Open API with Rate Limiter:** Accessible with rate limiting to prevent abuse.
2. **Authenticated API:** Provides unlimited access with proper authentication.
3. **Global Cache** Using Redis cache the api can scale horizontally.

### Prerequisites
- Rust and Cargo installed on your machine

### Open API with Rate Limiter
- **Path:** `/api/classify`
- **Open Path:** `/api/open/classify`
- **Method:** POST
- **Request:**
  ```json
   {
    "domain": "www.google.com", //Optional
    "cookies": [
      {
        "name": "_ga",
        "provider": "google.com"
      },
       {
        "name": "NID",
        "provider": "google.com"
      },
      {
        "name":"COOKIE",
        "provider":"unknown"
      }
    ]
  }
  ```

- **Response:**
```json
{
  "_ga": {
    "provider": "google.com",
    "category": "Statistics",
    "description": "ID used to identify users"
  },
  "NID": {
    "provider": "google.com",
    "category": "Marketing",
    "description": "This cookies is used to collect website statistics and track conversion rates and Google ad personalisation"
  }
}
```

### Build Docker image
```bash
docker build -t cookie-classifier .
```

