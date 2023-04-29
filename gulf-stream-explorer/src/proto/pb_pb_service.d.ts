// package: pb
// file: pb.proto

import * as pb_pb from "./pb_pb";
import {grpc} from "@improbable-eng/grpc-web";

type NodeSendBlock = {
  readonly methodName: string;
  readonly service: typeof Node;
  readonly requestStream: false;
  readonly responseStream: false;
  readonly requestType: typeof pb_pb.SendBlockRequest;
  readonly responseType: typeof pb_pb.GenericResponse;
};

type NodeSendTransaction = {
  readonly methodName: string;
  readonly service: typeof Node;
  readonly requestStream: false;
  readonly responseStream: false;
  readonly requestType: typeof pb_pb.SendTransactionRequest;
  readonly responseType: typeof pb_pb.GenericResponse;
};

type NodeGetHistory = {
  readonly methodName: string;
  readonly service: typeof Node;
  readonly requestStream: false;
  readonly responseStream: false;
  readonly requestType: typeof pb_pb.GetHistoryRequest;
  readonly responseType: typeof pb_pb.TransactionHistory;
};

type NodeGetBalance = {
  readonly methodName: string;
  readonly service: typeof Node;
  readonly requestStream: false;
  readonly responseStream: false;
  readonly requestType: typeof pb_pb.GetBalanceRequest;
  readonly responseType: typeof pb_pb.GetBalanceResponse;
};

export class Node {
  static readonly serviceName: string;
  static readonly SendBlock: NodeSendBlock;
  static readonly SendTransaction: NodeSendTransaction;
  static readonly GetHistory: NodeGetHistory;
  static readonly GetBalance: NodeGetBalance;
}

export type ServiceError = { message: string, code: number; metadata: grpc.Metadata }
export type Status = { details: string, code: number; metadata: grpc.Metadata }

interface UnaryResponse {
  cancel(): void;
}
interface ResponseStream<T> {
  cancel(): void;
  on(type: 'data', handler: (message: T) => void): ResponseStream<T>;
  on(type: 'end', handler: (status?: Status) => void): ResponseStream<T>;
  on(type: 'status', handler: (status: Status) => void): ResponseStream<T>;
}
interface RequestStream<T> {
  write(message: T): RequestStream<T>;
  end(): void;
  cancel(): void;
  on(type: 'end', handler: (status?: Status) => void): RequestStream<T>;
  on(type: 'status', handler: (status: Status) => void): RequestStream<T>;
}
interface BidirectionalStream<ReqT, ResT> {
  write(message: ReqT): BidirectionalStream<ReqT, ResT>;
  end(): void;
  cancel(): void;
  on(type: 'data', handler: (message: ResT) => void): BidirectionalStream<ReqT, ResT>;
  on(type: 'end', handler: (status?: Status) => void): BidirectionalStream<ReqT, ResT>;
  on(type: 'status', handler: (status: Status) => void): BidirectionalStream<ReqT, ResT>;
}

export class NodeClient {
  readonly serviceHost: string;

  constructor(serviceHost: string, options?: grpc.RpcOptions);
  sendBlock(
    requestMessage: pb_pb.SendBlockRequest,
    metadata: grpc.Metadata,
    callback: (error: ServiceError|null, responseMessage: pb_pb.GenericResponse|null) => void
  ): UnaryResponse;
  sendBlock(
    requestMessage: pb_pb.SendBlockRequest,
    callback: (error: ServiceError|null, responseMessage: pb_pb.GenericResponse|null) => void
  ): UnaryResponse;
  sendTransaction(
    requestMessage: pb_pb.SendTransactionRequest,
    metadata: grpc.Metadata,
    callback: (error: ServiceError|null, responseMessage: pb_pb.GenericResponse|null) => void
  ): UnaryResponse;
  sendTransaction(
    requestMessage: pb_pb.SendTransactionRequest,
    callback: (error: ServiceError|null, responseMessage: pb_pb.GenericResponse|null) => void
  ): UnaryResponse;
  getHistory(
    requestMessage: pb_pb.GetHistoryRequest,
    metadata: grpc.Metadata,
    callback: (error: ServiceError|null, responseMessage: pb_pb.TransactionHistory|null) => void
  ): UnaryResponse;
  getHistory(
    requestMessage: pb_pb.GetHistoryRequest,
    callback: (error: ServiceError|null, responseMessage: pb_pb.TransactionHistory|null) => void
  ): UnaryResponse;
  getBalance(
    requestMessage: pb_pb.GetBalanceRequest,
    metadata: grpc.Metadata,
    callback: (error: ServiceError|null, responseMessage: pb_pb.GetBalanceResponse|null) => void
  ): UnaryResponse;
  getBalance(
    requestMessage: pb_pb.GetBalanceRequest,
    callback: (error: ServiceError|null, responseMessage: pb_pb.GetBalanceResponse|null) => void
  ): UnaryResponse;
}

