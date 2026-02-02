import { aws_lambda as lambda } from "aws-cdk-lib";
import * as cdk from "aws-cdk-lib";
import { Construct } from "constructs";
import { getNamePrefixed } from "../../utils/prefix";
import { RustLambdaFunctionBuilder } from "../../constructs/lambda/rust-lambda-function-builder";
import { Environment } from "../../types/environment";
import { SRC_PATH } from "../../utils/constants";

export interface AgroStackProps extends cdk.StackProps {
    environment: Environment;
}

export class AgroStack extends cdk.Stack {
    constructor(scope: Construct, id: string, props: AgroStackProps) {
        super(scope, id, props);

        const { environment } = props;
        const removalPolicy =
            environment.envName === "prod"
                ? cdk.RemovalPolicy.RETAIN_ON_UPDATE_OR_DELETE
                : cdk.RemovalPolicy.DESTROY;

        /**
         * Settings
         */
        // const sourceAppName = `dev.luisk.${getNamePrefixed("", environment)}`;
        // const bucketName = getNamePrefixed("assets-bucket", environment);
        const awsLambdaWebAdapterLayerArn = `arn:aws:lambda:${this.region}:753240598075:layer:LambdaAdapterLayerArm64:25`; // Reference: https://github.com/awslabs/aws-lambda-web-adapter

        /**
         * Imports
         */

        /**
         * Resources
         */
        // const assetsBucket = new s3.Bucket(this, "AssetsBucket", {
        //     bucketName,
        //     removalPolicy: cdk.RemovalPolicy.DESTROY,
        //     autoDeleteObjects: true,
        //     accessControl: s3.BucketAccessControl.PRIVATE,
        //     blockPublicAccess: s3.BlockPublicAccess.BLOCK_ALL,
        //     publicReadAccess: false,
        //     objectOwnership: s3.ObjectOwnership.BUCKET_OWNER_ENFORCED,
        //     enforceSSL: true,
        //     cors: [
        //         {
        //             allowedMethods: [
        //                 s3.HttpMethods.GET,
        //                 s3.HttpMethods.DELETE,
        //                 s3.HttpMethods.PUT,
        //             ],
        //             allowedOrigins: [
        //                 "*", // @todo: Restrict later
        //             ],
        //             allowedHeaders: ["*"],
        //             exposedHeaders: [
        //                 "Access-Control-Allow-Headers",
        //                 "Access-Control-Allow-Methods",
        //                 "Access-Control-Allow-Origin",
        //                 "Access-Control-Max-Age",
        //             ],
        //             maxAge: cdk.Duration.minutes(5).toSeconds(),
        //         },
        //     ],
        // });

        // const hostedZone = route53.HostedZone.fromLookup(
        //     this,
        //     "CertificateHostedZone",
        //     {
        //         domainName: environment.domainName,
        //     },
        // );

        // const certificate = new acm.Certificate(this, "Certificate", {
        //     domainName: `*.${environment.envName}.${environment.domainName}`,
        //     subjectAlternativeNames: [
        //         `${environment.envName}.${environment.domainName}`,
        //         `login.${environment.envName}.${environment.domainName}`,
        //     ],
        //     validation: acm.CertificateValidation.fromDns(hostedZone),
        // });

        const awsLambdaWebAdapterLayer =
            lambda.LayerVersion.fromLayerVersionArn(
                this,
                "LambdaWebAdapterLayer",
                awsLambdaWebAdapterLayerArn,
            );

        const websiteLambda = new RustLambdaFunctionBuilder(
            this,
            "WebsiteLambda",
            {
                name: "website-lambda",
                environment,
            },
        )
            .withLogGroup()
            .withManifest("website", `${SRC_PATH}/packages`)
            .withEnvironmentVariables({
                RUST_LOG: environment.logLevel,
                AWS_LAMBDA_EXEC_WRAPPER: "/opt/bootstrap",
                AWS_LWA_READINESS_CHECK_PORT: "5173",
                AWS_LWA_PORT: "5173",
            })
            .withLayers([awsLambdaWebAdapterLayer])
            .build();

        const websiteLambdaFunctionUrl = websiteLambda.addFunctionUrl({
            authType: lambda.FunctionUrlAuthType.NONE,
        });

        /**
         * Exports
         */
        new cdk.CfnOutput(this, "ExportWebsiteLambdaUrl", {
            exportName: getNamePrefixed("website-lambda-url", environment),
            value: websiteLambdaFunctionUrl.url,
        });
    }
}
