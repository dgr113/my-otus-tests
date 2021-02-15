FROM python:3.9.1-alpine3.12


RUN apk add --no-cache build-base musl-dev zlib-dev libffi-dev libressl-dev

WORKDIR /usr/src/
COPY ./requirements.txt ./app/requirements.txt
RUN pip install --no-cache-dir -r ./app/requirements.txt

COPY src /usr/src/app


EXPOSE 80
ENTRYPOINT ["python", "-m", "app"]
