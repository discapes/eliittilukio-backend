# eliittilukio-backend

### Running locally

- Set the following variables in .env: DATABASE_URL,DATABASE_PASSWD,SMTP_USER,SMTP_PASS,JWT_KEY,ADMIN_EMAIL
  - Note that sqlx validates queries based on DATABASE_URL being available
- Start the development database with `podman-compose up`
- Fill it with the appropriate tables with `mysql -h::1 -uoe -pEhiekocoefohhiD2 oe < database.sql`
- `cargo run`
- `sudo caddy` if you plan on using the frontend

### Making changes

- Before committing, run `cargo sqlx prepare`
  - You might need to `cargo install sqlx-cli --no-default-features --features rustls,mysql`
  - This creates a .sqlx-folder, so that CI can build the container without a dev database
- Run cargo build to make sure the thing builds

### Deployment

- Build the container with `cargo sqlx prepare && podman build . -t eliittilukio-backend`
- Push it to ECR
  - if not logged in, `aws ecr get-login-password --region eu-north-1 | podman login --username AWS --password-stdin 861821644425.dkr.ecr.eu-north-1.amazonaws.com`
  - `podman tag eliittilukio-backend 861821644425.dkr.ecr.eu-north-1.amazonaws.com/eliittilukio-backend`
  - `podman push 861821644425.dkr.ecr.eu-north-1.amazonaws.com/eliittilukio-backend`
- On the server, fill the .env file and login to ECR
- Start the service with `docker compose pull && docker compose up -f compose-prod.yaml`
