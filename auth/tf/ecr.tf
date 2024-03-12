resource "aws_ecrpublic_repository" "ecr_auth" {
  provider = aws.us_east_1

  repository_name = "0bf0de84c3c818462fb872e32cd1471e_auth"
}