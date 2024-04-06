resource "aws_lambda_function" "lambda" {
  function_name = "untitled-frontend"
  timeout       = 5 # seconds

  image_uri     = "${aws_ecr_repository.ecr_frontend_private.repository_url}:latest"
  package_type  = "Image"

  role = aws_iam_role.lambda_execution_role.arn
}

resource "aws_cloudwatch_log_group" "lambda_log_group" {
  name              = "/aws/lambda/untitled-frontend"
  retention_in_days = 7
  lifecycle {
    prevent_destroy = false
  }
}

resource "aws_iam_role" "lambda_execution_role" {
  name = "untitled-site-lambda-execution-role-policy"

  assume_role_policy = jsonencode({
    Version = "2012-10-17"
    Statement = [
      {
        Action = "sts:AssumeRole",
        Effect = "Allow"
        Sid    = ""
        Principal = {
          Service = "lambda.amazonaws.com"
        }
      }
    ]
  })
}


resource "aws_iam_role_policy" "lambda_execution_role_policy" {
  name = "untitled-site-lambda-execution-role-policy"
  role = aws_iam_role.lambda_execution_role.id

  policy = jsonencode({
    Version = "2012-10-17"
    Statement = [
      {
        Effect = "Allow"
        Action = [
          "logs:CreateLogGroup",
          "logs:CreateLogStream",
          "logs:PutLogEvents"
        ]
        Resource = "arn:aws:logs:*:*:*"
      }
    ]
  })
}
