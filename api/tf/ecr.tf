resource "aws_ecrpublic_repository" "ecr_api" {
  provider = aws.us_east_1

  repository_name = "0bf0de84c3c818462fb872e32cd1471e_api"
}