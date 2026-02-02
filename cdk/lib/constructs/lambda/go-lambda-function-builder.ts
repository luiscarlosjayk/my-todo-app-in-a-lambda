import * as goLambda from '@aws-cdk/aws-lambda-go-alpha';
import * as cdk from 'aws-cdk-lib';
import * as lambda from 'aws-cdk-lib/aws-lambda';
import { Construct } from 'constructs';
import * as nodePath from 'node:path';
import { LambdaFunctionBuilder } from './lambda-function-builder.base';
import type { Environment } from '../../types/environment';
import { LAMBDA_BASEPATH } from '../../utils/constants';

export interface GoLambdaFunctionProps {
    name: string;
    environment: Environment;
    architecture?: lambda.Architecture;
}

export class GoLambdaFunctionBuilder extends LambdaFunctionBuilder {
    protected _lambda: goLambda.GoFunction;
    protected _entry: string;
    protected _moduleDir?: string;
    protected _bundling?: goLambda.BundlingOptions;
    protected _layers: lambda.ILayerVersion[] = [];
    protected _architecture: lambda.Architecture;

    constructor(scope: Construct, id: string, props: GoLambdaFunctionProps) {
        super(scope, id, props.name, props.environment);

        this._architecture = props.architecture ?? lambda.Architecture.ARM_64; // default

        // Defaults
        this.withEntry(this._name);
        this.withModuleDir(this._name);
    }

    withEntry(path: string, basePath?: string): this {
        basePath = basePath ?? LAMBDA_BASEPATH;
        this._entry = nodePath.join(basePath, path, 'main.go');
        return this;
    }

    withModuleDir(path: string, basePath?: string): this {
        basePath = basePath ?? LAMBDA_BASEPATH;
        this._moduleDir = nodePath.join(basePath, path, 'go.mod');
        return this;
    }

    withBundling(bundling: goLambda.BundlingOptions): this {
        this._bundling = bundling;
        return this;
    }

    withLayers(layers: lambda.ILayerVersion[]): this {
        this._layers.push(...layers);
        return this;
    }

    build(): cdk.aws_lambda.IFunction {
        if (!this._entry) {
            throw 'Expected entry to be defined.';
        }

        if (!this._moduleDir) {
            throw 'Expected moduleDir to be defined.';
        }

        this._lambda = new goLambda.GoFunction(this, `GoLambda${this._id}`, {
            functionName: this._lambdaName,
            entry: this._entry,
            moduleDir: this._moduleDir,
            timeout: this._duration,
            memorySize: this._memorySize,
            logGroup: this._logGroup,
            environment: this._environmentVariables,
            reservedConcurrentExecutions: this._concurrency,
            architecture: this._architecture,
            role: this._role,
            vpc: this._vpc,
            vpcSubnets: this._vpcSubnets,
            securityGroups: this._securityGroups,
            layers: this._layers,
            bundling: this._bundling,
            allowPublicSubnet: this._allowPublicSubnet,
            runtime: this._runtime,
            systemLogLevelV2: this._systemLogLevel,
            applicationLogLevelV2: this._applicationLogLevel,
            loggingFormat: this._loggingFormat,
        });

        return this._lambda;
    }
}
