resource "aws_ecrpublic_repository" "ecr_auth_web" {
  provider = aws.us_east_1

  repository_name = "d59985bd7579cc56b43532c0c04135b1_auth_web"
}