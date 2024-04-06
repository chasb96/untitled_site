resource "aws_ecrpublic_repository" "ecr_frontend" {
  provider = aws.us_east_1

  repository_name = "0d26b081810df667c5aadb4638255b37_frontend"
}

resource "aws_ecr_repository" "ecr_frontend_private" {
  provider = aws.us_east_1

  name = "0750d84b0e1516450278d5985b5fb55a_frontend"
}