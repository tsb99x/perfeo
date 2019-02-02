FROM node

ENV NODE_ENV production

WORKDIR /usr/src/http_serv_perf
COPY . .
RUN npm install --only=production
EXPOSE 8080

CMD ["npm", "start"]
