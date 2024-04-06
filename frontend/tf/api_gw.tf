resource "aws_apigatewayv2_api" "gateway" {
  name          = "untitled_site_gateway"
  protocol_type = "HTTP"
}

resource "aws_apigatewayv2_stage" "gateway_stage" {
  api_id = aws_apigatewayv2_api.gateway.id

  name        = "$default"
  auto_deploy = true
}

resource "aws_apigatewayv2_integration" "lambda" {
  api_id           = aws_apigatewayv2_api.gateway.id
  integration_type = "AWS_PROXY"

  integration_method = "POST"
  integration_uri    = aws_lambda_function.lambda.invoke_arn
}

resource "aws_apigatewayv2_route" "index" {
  api_id = aws_apigatewayv2_api.gateway.id

  route_key = "GET /"
  target    = "integrations/${aws_apigatewayv2_integration.lambda.id}"
}

resource "aws_apigatewayv2_route" "pathed" {
  api_id = aws_apigatewayv2_api.gateway.id

  route_key = "GET /{path+}"
  target    = "integrations/${aws_apigatewayv2_integration.lambda.id}"
}

resource "aws_lambda_permission" "invoke_lambda" {
  statement_id  = "AllowExecutionFromAPIGateway"
  action        = "lambda:InvokeFunction"
  function_name = aws_lambda_function.lambda.function_name
  principal     = "apigateway.amazonaws.com"

  source_arn = "${aws_apigatewayv2_api.gateway.execution_arn}/*/*"
}