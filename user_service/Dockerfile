FROM golang

WORKDIR /app

COPY go.mod go.sum ./
RUN go mod download

COPY ./cmd ./cmd

RUN CGO_ENABLED=0 GOOS=linux go build -o /user_service ./cmd

EXPOSE 80

CMD ["/user_service"]