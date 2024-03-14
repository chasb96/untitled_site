resource "aws_ecrpublic_repository" "ecr_users_migrations" {
  provider = aws.us_east_1

  repository_name = "7b1a10ee2186b7f5ee795dcfeae72f57_users_migrations"
}