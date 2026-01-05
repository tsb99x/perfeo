FROM docker.io/library/node:14

WORKDIR /usr/src/perfeo

COPY package*.json ./
RUN npm install --only=production
COPY . .

EXPOSE 8080

CMD [ "npm", "start" ]

HEALTHCHECK --interval=5s --timeout=30s --retries=3 \
  CMD curl --fail http://localhost:8080/ping || exit 1
