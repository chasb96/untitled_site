FROM golang

WORKDIR /app

COPY go.mod go.sum ./
RUN go mod download

COPY ./src ./src

RUN CGO_ENABLED=0 GOOS=linux go build -o ./projects_web ./src

EXPOSE 80

CMD ["./projects_web"]