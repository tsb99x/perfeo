FROM docker.io/library/python:3.13

WORKDIR /usr/src/perfeo

COPY requirements.txt ./
RUN pip install --no-cache-dir -r requirements.txt
COPY . .

CMD [ "python", "./main.py" ]

HEALTHCHECK --interval=5s --timeout=30s --retries=3 \
  CMD curl --fail http://localhost:8080/ping || exit 1
