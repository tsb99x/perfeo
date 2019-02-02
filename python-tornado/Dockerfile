FROM python:3

WORKDIR /usr/src/http_serv_perf
COPY . .
RUN pip install --no-cache-dir -r requirements.txt

CMD [ "python", "./main.py" ]
