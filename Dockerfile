FROM python:3-alpine3.12

RUN apk update
RUN apk add --no-cache docker-cli docker-compose bash

COPY ./app /app
WORKDIR /app
RUN pip install -r requirements.txt

EXPOSE 8080/tcp

CMD ["flask", "run", "-h", "0.0.0.0", "-p", "5000"]