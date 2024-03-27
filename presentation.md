---
marp: true
author: Pieter Engelbrecht
style: pre { font-size: 40px }
---

<style>
@font-face {
  font-family:"Formula Condensed";
  src: url(https://www.rustnationuk.com/assets/fonts/formula-condensed/PPFormula-CondensedBold.woff2);
}
h1,h2 {
  font-family:"Formula Condensed",sans-serif;
  color: #fedb30;
}
h1 { font-size: 3em; }
h2 { font-size: 2em; }
:root {
  --color-fg-default: #fff;
  background-image: url(static/rust-background.png);
  font-size: 2em;
}
img {
  background-color: transparent;
}
header, footer {
  right: 30px;
  left: auto;
}
.shuttle {
  color: #F25100
}
a {
  color: #fedb30;
}
</style>

# Creating a microservice app with <span class="shuttle">Shuttle</span>

... well, how to guard access with JWTs

---
<!-- header: ![image](static/rust-logo.svg) -->
<!-- footer: ![height:61px](static/shuttle-logo.png) -->

## About me
- Pieter Engelbrecht
- But known as **chesedo** on online forums
- Tech lead at **Shuttle**

---

## Layout
Interactive tutorial
Interleave theory
Ask questions at any time

https://github.com/chesedo/rust-nation

---

<style>
figure {
  margin-left: 4em !important;
}
</style>

![bg left contain](static/services.svg)

- Gateway
  - /order -> Order service
  - /user -> User service

---

<!-- Not sending it in request params -->

## What is a JWT anyway?
Is used for authorization
JSON Web Token is made up of 3 parts:
1. Header
2. Payload
3. Signature (base64(header) . base64(payload))

---

<style>
  .header {
    color: #f94b4f;

  }
  .payload {
    color: #fedb30;
  }
  .signature {
    color: #00bb74;
  }
  pre {
    background: none;
    border: 2px solid white;
    font-size: 1.2em;
  }
  .grid {
    display: flex;
    gap: 3em;
  }
</style>

<div class="grid">
<div>

### Header
<pre class="header">
{
  "alg": "HS256",
  "typ": "JWT"
}
</pre>
</div>

<div>

### Payload
<!-- Should never contain sensitive information -->
<pre class="payload">
{
  "sub": "1234567890",
  "name": "John Doe",
  "iat": 1516239022
}
</pre>
</div>
</div>

### Signature
<span class="header">eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9</span>.<span class="payload">eyJzdWIiOiIxMjM0NTY3ODkwIiwibmFtZSI6IkpvaG4gRG9lIiwiaWF0IjoxNTE2MjM5MDIyfQ</span> => <span class="signature">XbPfbIHMI6arZ3Y922BhjWgQzWXcXNrz0ogtVhfEd2o</span>

---

## Payload fields
<div class="grid">
<div>

<!-- 
Why short expiry
Keep the payload compact still
 -->

- Registered (RFC 7519)
  - iss = Issuer
  - sub = Subject
  - aud = Audience
  - exp = Expiration Time
  - nbf = Not Before
  - iat = Issued At
  - jti = JWT ID
</div>
<div>

- Public
- Private
</div>
</div>

---

![bg left contain](static/services.svg)

- Gateway
  - /order -> Order service
  - /user -> User service


---

## Best practices
Keep the payload compact
Short expiry
Don't have sensitive information in the payload
Don't send the token in the request params
Not ideal for session management

---
<!-- header: '' -->
<!-- footer: '' -->

# Questions?

https://github.com/chesedo/rust-nation