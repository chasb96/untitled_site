resource "aws_ecrpublic_repository" "ecr_files_migrations" {
  provider = aws.us_east_1

  repository_name = "bfb61b46c7b497a70d89503e3f401cf0_files_migrations"
}