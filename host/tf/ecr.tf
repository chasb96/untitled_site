resource "aws_ecrpublic_repository" "ecr_host" {
  provider = aws.us_east_1

  repository_name = "f9ba36233aa813e998494e53ac41efa0_host"
}