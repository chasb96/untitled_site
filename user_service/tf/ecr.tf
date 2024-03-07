resource "aws_ecrpublic_repository" "ecr_user_service" {
  provider = aws.us_east_1

  repository_name = "689289bfd7b0db6b706123cfa0b95825_user_service"
}