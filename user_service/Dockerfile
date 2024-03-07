FROM golang

WORKDIR /app

COPY go.mod go.sum ./
RUN go mod download

WORKDIR /app/cmd
COPY ./cmd/*.go ./

RUN CGO_ENABLED=0 GOOS=linux go build -o /user_service

EXPOSE 80

CMD ["/user_service"]