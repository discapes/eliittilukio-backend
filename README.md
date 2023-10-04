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

- Prepare the sqlx queries: `cargo sqlx prepare`

- Build and push backend:

```bash
DEST=861821644425.dkr.ecr.eu-north-1.amazonaws.com
aws ecr get-login-password --region eu-north-1 | docker login --username AWS --password-stdin $DEST
docker build . -t $DEST/eliittilukio-backend; docker push $DEST/eliittilukio-backend
```

- On the server:
  - `sudo apt update && sudo apt install vim git docker tmux docker-compose awscli mariadb-client`
  - fill the .env file and login to ECR
  - Get updates: `docker-compose -f compose-prod.yaml pull`
  - Using tmux: run with `tmux a`, new window,`C-b c`, detach `C-b d`, change window `C-b <0-9>`
  - Start the servicess with `docker-compose -f compose-prod.yaml up --abort-on-container-exit`
