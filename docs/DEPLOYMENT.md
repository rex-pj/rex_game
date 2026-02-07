# Rex Game - Deployment Guide

This guide walks you through deploying the Rex Game application to Google Compute Engine with CI/CD using GitHub Actions.

## Table of Contents

1. [Architecture Overview](#architecture-overview)
2. [Prerequisites](#prerequisites)
3. [Part 1: Create Compute Engine Instance](#part-1-create-compute-engine-instance)
4. [Part 2: Server Setup](#part-2-server-setup)
5. [Part 3: Email Service Configuration](#part-3-email-service-configuration)
6. [Part 4: Production Configuration](#part-4-production-configuration)
7. [Part 5: CI/CD with GitHub Actions](#part-5-cicd-with-github-actions) *(includes passwordless sudo setup)*
8. [Part 6: Monitoring and Maintenance](#part-6-monitoring-and-maintenance)
9. [Troubleshooting](#troubleshooting)
10. [Cost Estimation](#cost-estimation)

---

## Architecture Overview

```
┌─────────────────┐      ┌─────────────────┐      ┌─────────────────────────────┐
│   GitHub Repo   │ ──── │  GitHub Actions │ ──── │    Google Compute Engine    │
│                 │      │     (CI/CD)     │      │         (Ubuntu VM)         │
└─────────────────┘      └─────────────────┘      └─────────────────────────────┘
                                                              │
                         ┌────────────────────────────────────┼────────────────┐
                         │                                    │                │
                   ┌─────┴─────┐                        ┌─────┴─────┐    ┌─────┴─────┐
                   │  Nginx    │                        │ PostgreSQL│    │  Uploads  │
                   │ (Reverse  │                        │  (Local)  │    │  Storage  │
                   │  Proxy)   │                        └───────────┘    └───────────┘
                   └─────┬─────┘
                         │
          ┌──────────────┼──────────────┐
          │              │              │
    ┌─────┴─────┐  ┌─────┴─────┐  ┌─────┴─────┐
    │  Frontend │  │  Backend  │  │  Static   │
    │ SvelteKit │  │ Rust/Axum │  │  Files    │
    │  :3000    │  │   :8080   │  │ /uploads  │
    └───────────┘  └───────────┘  └───────────┘
```

### Technology Stack

| Component | Technology |
|-----------|------------|
| Frontend | SvelteKit 2.x with Svelte 5 |
| Backend | Rust with Axum framework |
| Database | PostgreSQL 15 |
| ORM | SeaORM |
| Web Server | Nginx (reverse proxy) |
| SSL | Let's Encrypt (Certbot) |
| CI/CD | GitHub Actions |
| Email | Resend |

---

## Prerequisites

Before starting, ensure you have:

- [ ] Google Cloud Platform account with billing enabled
- [ ] Domain name (optional but recommended for SSL)
- [ ] GitHub repository with the Rex Game code
- [ ] Basic knowledge of Linux command line

---

## Part 1: Create Compute Engine Instance

### 1.1 Create VM Instance

1. Go to [Google Cloud Console](https://console.cloud.google.com)
2. Navigate to **Compute Engine** > **VM instances**
3. Click **Create Instance**

**Recommended Configuration (Free Tier Eligible):**

| Setting | Value |
|---------|-------|
| Name | `rex-game-server` |
| Region | `us-central1` (or `us-east1`, `us-west1`) |
| Zone | `us-central1-a` |
| Machine type | `e2-micro` (Free tier) |
| Boot disk | Ubuntu 22.04 LTS, 30GB SSD |
| Firewall | ✅ Allow HTTP, ✅ Allow HTTPS |

### 1.2 Configure Firewall Rules

Navigate to **VPC Network** > **Firewall** > **Create Firewall Rule**

**Rule 1: Allow HTTP/HTTPS (required for SSL certificate)**

| Setting | Value |
|---------|-------|
| Name | `allow-http-https` |
| Targets | All instances in the network |
| Source IP ranges | `0.0.0.0/0` |
| Protocols and ports | TCP: `80, 443` |

**Rule 2: Allow Backend API**

| Setting | Value |
|---------|-------|
| Name | `allow-backend-api` |
| Targets | All instances in the network |
| Source IP ranges | `0.0.0.0/0` |
| Protocols and ports | TCP: `8080` |

### 1.3 Reserve Static IP (Optional but Recommended)

1. Go to **VPC Network** > **IP addresses**
2. Click **Reserve External Static Address**
3. Attach to your VM instance

### 1.4 Connect via SSH

```bash
gcloud compute ssh rex-game-server --zone=us-central1-a
```

---

## Part 2: Server Setup

### 2.1 Update System and Install Base Packages

```bash
# Update package lists
sudo apt update && sudo apt upgrade -y

# Install essential packages
sudo apt install -y \
    build-essential \
    pkg-config \
    libssl-dev \
    curl \
    git \
    nginx \
    certbot \
    python3-certbot-nginx \
    ufw
```

### 2.2 Install PostgreSQL

```bash
# Install PostgreSQL 15
sudo apt install -y postgresql postgresql-contrib

# Start and enable PostgreSQL
sudo systemctl start postgresql
sudo systemctl enable postgresql

# Verify installation
sudo systemctl status postgresql
```

**Create Database and User:**

```bash
# Switch to postgres user and create database
sudo -u postgres psql << 'EOF'
-- Create user with password
CREATE USER rex_user WITH PASSWORD 'your_secure_password_here';

-- Create database
CREATE DATABASE rex_game_db OWNER rex_user;

-- Grant privileges
GRANT ALL PRIVILEGES ON DATABASE rex_game_db TO rex_user;

-- Exit
\q
EOF
```

**Test Connection:**

```bash
psql -h localhost -U rex_user -d rex_game_db -c "SELECT version();"
```

### 2.3 Install Rust

```bash
# Install Rust using rustup
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Select option 1 (default installation)

# Load Rust environment
source $HOME/.cargo/env

# Verify installation
rustc --version
cargo --version
```

### 2.4 Install Node.js

```bash
# Install NVM (Node Version Manager)
curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.0/install.sh | bash

# Load NVM
source ~/.bashrc

# Install Node.js LTS
nvm install --lts

# Verify installation
node --version
npm --version
```

### 2.5 Configure Firewall (UFW)

```bash
# Enable UFW
sudo ufw enable

# Allow SSH
sudo ufw allow OpenSSH

# Allow HTTP and HTTPS
sudo ufw allow 'Nginx Full'

# Allow backend port (for internal use)
sudo ufw allow 8080/tcp

# Check status
sudo ufw status
```

---

## Part 3: Email Service Configuration

### 3.1 Setting Up Resend

1. Sign up at [https://resend.com](https://resend.com) (free tier: 3,000 emails/month)
2. Verify your domain
3. Generate API Key
4. Set environment variables:

```bash
EMAIL_PROVIDER=resend
RESEND_API_KEY=re_xxxxxxxxxxxxxxxxxxxxxxxxxx
EMAIL_FROM_ADDRESS=noreply@yourdomain.com
EMAIL_FROM_NAME=Rex Game
```

---

## Part 4: Production Configuration

### 4.1 Create Application Directories

```bash
# Create directory structure
sudo mkdir -p /var/www/rex-game/{backend,client-app,uploads}
sudo mkdir -p /var/www/rex-game/backend/{target/release,environments}

# Set ownership
sudo chown -R $USER:$USER /var/www/rex-game

# Set permissions for uploads
sudo chmod 755 /var/www/rex-game/uploads
```

### 4.2 Create Production Environment File

```bash
nano /var/www/rex-game/backend/environments/.env.prod
```

**Contents of `.env.prod`:**

```bash
# ============================================================
# Rex Game Backend - Production Environment
# ============================================================

# Database
DATABASE_URL=postgres://rex_user:your_secure_password@localhost:5432/rex_game_db

# JWT Configuration
JWT_CLIENT_SECRET=your-super-secure-jwt-secret-minimum-32-characters-long
JWT_EXPIRATION=3600
JWT_REFRESH_EXPIRATION=604800

# CORS (replace with your actual domain)
CORS_ALLOW_ORIGINS=https://yourdomain.com,https://www.yourdomain.com

# Email Configuration
EMAIL_PROVIDER=resend
RESEND_API_KEY=re_xxxxxxxxxxxxxxxxxxxxxxxxxx
EMAIL_FROM_ADDRESS=noreply@yourdomain.com
EMAIL_FROM_NAME=Rex Game

# Platform URLs
PLATFORM_URL=https://yourdomain.com
SIGNUP_VERIFICATION_URL=https://yourdomain.com/account/confirm?token=[token]
RESET_PASSWORD_URL=https://yourdomain.com/account/reset-password?token=[token]

# Server Configuration
SERVER_HOST=127.0.0.1
SERVER_PORT=8080
```

### 4.3 Configure Nginx (HTTP)

First, create an HTTP-only config. SSL will be added automatically by Certbot in the next step.

```bash
sudo nano /etc/nginx/sites-available/rex-game
```

```nginx
server {
    listen 80;
    listen [::]:80;
    server_name yourdomain.com www.yourdomain.com;

    # Security Headers
    add_header X-Frame-Options "SAMEORIGIN" always;
    add_header X-Content-Type-Options "nosniff" always;
    add_header X-XSS-Protection "1; mode=block" always;

    # Gzip Compression
    gzip on;
    gzip_vary on;
    gzip_min_length 1024;
    gzip_types text/plain text/css text/xml text/javascript application/javascript application/json application/xml;

    # Frontend (SvelteKit)
    location / {
        proxy_pass http://127.0.0.1:3000;
        proxy_http_version 1.1;
        proxy_set_header Upgrade $http_upgrade;
        proxy_set_header Connection 'upgrade';
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
        proxy_cache_bypass $http_upgrade;
        proxy_read_timeout 60s;
        proxy_connect_timeout 60s;
    }

    # Backend API
    location /api/ {
        proxy_pass http://127.0.0.1:8080/;
        proxy_http_version 1.1;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
        proxy_read_timeout 60s;
        proxy_connect_timeout 60s;

        # Handle preflight requests
        if ($request_method = OPTIONS) {
            add_header Access-Control-Allow-Origin $http_origin always;
            add_header Access-Control-Allow-Methods "GET, POST, PUT, PATCH, DELETE, OPTIONS" always;
            add_header Access-Control-Allow-Headers "Authorization, Content-Type, Accept" always;
            add_header Access-Control-Max-Age 86400;
            return 204;
        }
    }

    # Static Uploads
    location /uploads/ {
        alias /var/www/rex-game/uploads/;
        expires 30d;
        add_header Cache-Control "public, immutable";

        # Security: prevent script execution
        location ~* \.(php|pl|py|jsp|asp|sh|cgi)$ {
            deny all;
        }
    }

    # Health Check Endpoint
    location /health {
        access_log off;
        return 200 "OK";
        add_header Content-Type text/plain;
    }
}
```

**Enable the site:**

```bash
# Create symbolic link
sudo ln -s /etc/nginx/sites-available/rex-game /etc/nginx/sites-enabled/

# Remove default site
sudo rm /etc/nginx/sites-enabled/default

# Test configuration
sudo nginx -t

# Reload Nginx
sudo systemctl reload nginx
```

### 4.4 SSL Certificate with Let's Encrypt

**Note for Cloudflare users:** Before running Certbot, temporarily disable **Always Use HTTPS** on Cloudflare:
1. Go to **Cloudflare Dashboard** > domain > **SSL/TLS** > **Edge Certificates**
2. Find **Always Use HTTPS** > turn **OFF**

**Run Certbot:**

```bash
# Obtain SSL certificate (Certbot auto-updates Nginx config)
sudo certbot --nginx -d yourdomain.com -d www.yourdomain.com

# Verify Nginx config after Certbot changes
sudo nginx -t

# Verify auto-renewal
sudo certbot renew --dry-run

# Enable auto-renewal timer
sudo systemctl enable certbot.timer
```

**After Certbot succeeds:**
1. Re-enable **Always Use HTTPS** on Cloudflare
2. Go to **SSL/TLS** > **Overview** > set mode to **Full (Strict)**

### 4.5 Create Systemd Services

**Backend Service:**

```bash
sudo nano /etc/systemd/system/rex-backend.service
```

```ini
[Unit]
Description=Rex Game Backend API
After=network.target postgresql.service
Wants=postgresql.service

[Service]
Type=simple
User=www-data
Group=www-data
WorkingDirectory=/var/www/rex-game/backend
Environment="APP_ENV=prod"
Environment="RUST_LOG=info"
ExecStart=/var/www/rex-game/backend/target/release/rex_game
Restart=always
RestartSec=5
StandardOutput=journal
StandardError=journal

# Security hardening
NoNewPrivileges=true
ProtectSystem=strict
ProtectHome=true
ReadWritePaths=/var/www/rex-game/uploads

[Install]
WantedBy=multi-user.target
```

**Frontend Service:**

```bash
sudo nano /etc/systemd/system/rex-frontend.service
```

```ini
[Unit]
Description=Rex Game Frontend (SvelteKit)
After=network.target

[Service]
Type=simple
User=www-data
Group=www-data
WorkingDirectory=/var/www/rex-game/client-app
Environment="NODE_ENV=production"
Environment="PORT=3000"
Environment="HOST=127.0.0.1"
ExecStart=/home/YOUR_USER/.nvm/versions/node/v20.x.x/bin/node build
Restart=always
RestartSec=5
StandardOutput=journal
StandardError=journal

[Install]
WantedBy=multi-user.target
```

> **Note:** Replace `YOUR_USER` and `v20.x.x` with your actual username and Node version.
> Find the correct path with: `which node`

**Enable and start services:**

```bash
# Reload systemd
sudo systemctl daemon-reload

# Enable services to start on boot
sudo systemctl enable rex-backend rex-frontend

# Start services
sudo systemctl start rex-backend rex-frontend

# Check status
sudo systemctl status rex-backend rex-frontend
```

---

## Part 5: CI/CD with GitHub Actions

### 5.1 Generate SSH Key for Deployment

**On your server:**

```bash
# Generate SSH key pair
ssh-keygen -t ed25519 -C "github-actions-deploy" -f ~/.ssh/github_deploy -N ""

# Add public key to authorized_keys
cat ~/.ssh/github_deploy.pub >> ~/.ssh/authorized_keys

# Set correct permissions
chmod 600 ~/.ssh/authorized_keys
chmod 700 ~/.ssh

# Display private key (you'll need this for GitHub)
cat ~/.ssh/github_deploy
```

### 5.2 Configure GitHub Secrets

Go to your GitHub repository:
**Settings** > **Secrets and variables** > **Actions** > **New repository secret**

Add the following secrets:

| Secret Name | Description | Example |
|-------------|-------------|---------|
| `SSH_PRIVATE_KEY` | Content of `~/.ssh/github_deploy` | `-----BEGIN OPENSSH PRIVATE KEY-----...` |
| `SSH_HOST` | Server IP or hostname | `34.123.45.67` or `yourdomain.com` |
| `SSH_USER` | SSH username | `your_username` |

### 5.3 Configure Passwordless Sudo for Deploy User

The CI/CD pipeline uses `sudo` to manage services and files on the server. Configure limited passwordless sudo for the deploy user:

```bash
# On the server, run:
sudo visudo -f /etc/sudoers.d/deploy-user
```

Add the following (replace `your_username` with the SSH_USER):

```
your_username ALL=(ALL) NOPASSWD: /usr/bin/systemctl, /usr/bin/mkdir, /usr/bin/mv, /usr/bin/cp, /usr/bin/rm, /usr/bin/chown, /usr/bin/chmod, /usr/bin/tar
```

```bash
# Verify it works
sudo -n systemctl status nginx
```

> **Security note:** This only grants passwordless sudo for specific commands needed by the deploy pipeline, not full root access.

### 5.4 GitHub Actions Workflow

The workflow file is already created at `.github/workflows/deploy.yml`. It includes:

1. **Build Backend Job:**
   - Compiles Rust code in release mode
   - Caches dependencies for faster builds
   - Uploads binary as artifact

2. **Build Frontend Job:**
   - Installs Node.js dependencies
   - Builds SvelteKit application
   - Uploads build output as artifact

3. **Deploy Job:**
   - Downloads artifacts from build jobs
   - Connects to server via SSH
   - Deploys backend binary
   - Deploys frontend build
   - Runs health check
   - Automatic rollback on failure

### 5.5 Trigger Deployment

Deployment is triggered automatically when:
- Code is pushed to `main` or `master` branch
- Manual trigger via **Actions** > **Deploy to Production** > **Run workflow**

---

## Part 6: Monitoring and Maintenance

### 6.1 View Logs

```bash
# Backend logs
sudo journalctl -u rex-backend -f

# Frontend logs
sudo journalctl -u rex-frontend -f

# Nginx access logs
sudo tail -f /var/log/nginx/access.log

# Nginx error logs
sudo tail -f /var/log/nginx/error.log

# PostgreSQL logs
sudo tail -f /var/log/postgresql/postgresql-15-main.log
```

### 6.2 Database Backup

**Create backup script:**

```bash
sudo nano /usr/local/bin/backup-db.sh
```

```bash
#!/bin/bash
BACKUP_DIR="/var/backups/postgresql"
TIMESTAMP=$(date +%Y%m%d_%H%M%S)
BACKUP_FILE="$BACKUP_DIR/rex_game_db_$TIMESTAMP.sql.gz"

# Create backup directory
mkdir -p $BACKUP_DIR

# Create backup
sudo -u postgres pg_dump rex_game_db | gzip > $BACKUP_FILE

# Delete backups older than 7 days
find $BACKUP_DIR -type f -mtime +7 -delete

echo "Backup created: $BACKUP_FILE"
```

```bash
# Make executable
sudo chmod +x /usr/local/bin/backup-db.sh

# Add to crontab (daily at 2 AM)
sudo crontab -e
# Add: 0 2 * * * /usr/local/bin/backup-db.sh
```

### 6.3 SSL Certificate Renewal

Let's Encrypt certificates auto-renew. Verify with:

```bash
# Check renewal timer
sudo systemctl status certbot.timer

# Test renewal
sudo certbot renew --dry-run
```

### 6.4 System Updates

```bash
# Update system packages
sudo apt update && sudo apt upgrade -y

# Reboot if kernel was updated
sudo reboot
```

---

## Troubleshooting

### Backend won't start

```bash
# Check logs
sudo journalctl -u rex-backend -n 50

# Verify environment file
cat /var/www/rex-game/backend/environments/.env.prod

# Test database connection
psql -h localhost -U rex_user -d rex_game_db

# Check file permissions
ls -la /var/www/rex-game/backend/target/release/
```

### Frontend won't start

```bash
# Check logs
sudo journalctl -u rex-frontend -n 50

# Verify build exists
ls -la /var/www/rex-game/client-app/build/

# Check Node.js path in service file
which node
```

### Nginx errors

```bash
# Test configuration
sudo nginx -t

# Check error log
sudo tail -f /var/log/nginx/error.log

# Verify upstream services are running
curl http://127.0.0.1:3000  # Frontend
curl http://127.0.0.1:8080  # Backend
```

### SSL certificate issues

```bash
# Check certificate status
sudo certbot certificates

# Force renewal
sudo certbot renew --force-renewal

# Check Nginx SSL configuration
sudo nginx -t

# If using Cloudflare, verify SSL mode is "Full (Strict)"
```

### Database connection issues

```bash
# Check PostgreSQL status
sudo systemctl status postgresql

# Check if database exists
sudo -u postgres psql -c "\l" | grep rex_game_db

# Check user permissions
sudo -u postgres psql -c "\du" | grep rex_user

# Reset user password if needed
sudo -u postgres psql -c "ALTER USER rex_user WITH PASSWORD 'new_password';"
```

---

## Cost Estimation

### Monthly Costs (USD)

| Item | Free Tier | After Free Tier |
|------|-----------|-----------------|
| Compute Engine (e2-micro) | $0 (720 hrs/month) | ~$6.11 |
| Static IP | $0 (when attached) | ~$2.88 (when not attached) |
| Egress (first 1GB) | Free | $0.12/GB |
| PostgreSQL (local) | $0 | $0 |
| Disk (30GB SSD) | Free (up to 30GB) | ~$1.20 |
| Email (Resend) | $0 (3k emails/month) | $20 (50k/month) |
| **Total** | **$0** | **~$10-15** |

### Free Tier Eligibility

Google Cloud offers:
- 1 `e2-micro` instance per month (in select regions)
- 30 GB standard persistent disk
- 1 GB network egress

To stay within free tier:
- Use `e2-micro` machine type
- Stay in `us-central1`, `us-east1`, or `us-west1`
- Keep disk under 30GB

---

## Quick Reference

### Service Commands

```bash
# Backend
sudo systemctl start rex-backend
sudo systemctl stop rex-backend
sudo systemctl restart rex-backend
sudo systemctl status rex-backend

# Frontend
sudo systemctl start rex-frontend
sudo systemctl stop rex-frontend
sudo systemctl restart rex-frontend
sudo systemctl status rex-frontend

# Nginx
sudo systemctl reload nginx
sudo nginx -t

# PostgreSQL
sudo systemctl restart postgresql
```

### Useful Paths

| Path | Description |
|------|-------------|
| `/var/www/rex-game/backend/` | Backend application |
| `/var/www/rex-game/client-app/` | Frontend application |
| `/var/www/rex-game/uploads/` | User uploads |
| `/var/www/rex-game/backend/environments/.env.prod` | Production config |
| `/etc/nginx/sites-available/rex-game` | Nginx config |
| `/etc/systemd/system/rex-*.service` | Systemd services |

### Environment Variables

| Variable | Description |
|----------|-------------|
| `APP_ENV` | Environment name (`dev`, `prod`) |
| `DATABASE_URL` | PostgreSQL connection string |
| `JWT_CLIENT_SECRET` | Secret for JWT signing |
| `EMAIL_PROVIDER` | Email provider (`resend`) |
| `CORS_ALLOW_ORIGINS` | Allowed origins for CORS |

---

## Support

For issues or questions:
- Open an issue on the [GitHub repository](https://github.com/rex-pj/rex_game/issues)
- Check the [troubleshooting section](#troubleshooting)

---

*Last updated: February 2026*
