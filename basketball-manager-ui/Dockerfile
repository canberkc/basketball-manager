FROM node:14.17.1-slim as build-step

RUN mkdir -p /app
WORKDIR /app

COPY package.json /app
RUN npm install

COPY . /app
RUN npm run build --prod


FROM nginx:1.21-alpine

COPY nginx.conf /etc/nginx/nginx.conf
COPY --from=build-step /app/dist/basketball-manager-ui /usr/share/nginx/html

CMD [ "nginx", "-g", "daemon off;" ]
