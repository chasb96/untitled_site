resource "aws_ecrpublic_repository" "ecr_frontend" {
  provider = aws.us_east_1

  repository_name = "0d26b081810df667c5aadb4638255b37_frontend"
}