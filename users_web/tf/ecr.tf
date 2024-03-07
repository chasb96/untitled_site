resource "aws_ecrpublic_repository" "ecr_users_web" {
  provider = aws.us_east_1

  repository_name = "689289bfd7b0db6b706123cfa0b95825_users_web"
}