// package: pb
// file: pb.proto

var pb_pb = require("./pb_pb");
var grpc = require("@improbable-eng/grpc-web").grpc;

var Node = (function () {
  function Node() {}
  Node.serviceName = "pb.Node";
  return Node;
}());

Node.SendBlock = {
  methodName: "SendBlock",
  service: Node,
  requestStream: false,
  responseStream: false,
  requestType: pb_pb.SendBlockRequest,
  responseType: pb_pb.GenericResponse
};

Node.SendTransaction = {
  methodName: "SendTransaction",
  service: Node,
  requestStream: false,
  responseStream: false,
  requestType: pb_pb.SendTransactionRequest,
  responseType: pb_pb.GenericResponse
};

Node.GetHistory = {
  methodName: "GetHistory",
  service: Node,
  requestStream: false,
  responseStream: false,
  requestType: pb_pb.GetHistoryRequest,
  responseType: pb_pb.TransactionHistory
};

Node.GetBalance = {
  methodName: "GetBalance",
  service: Node,
  requestStream: false,
  responseStream: false,
  requestType: pb_pb.GetBalanceRequest,
  responseType: pb_pb.GetBalanceResponse
};

exports.Node = Node;

function NodeClient(serviceHost, options) {
  this.serviceHost = serviceHost;
  this.options = options || {};
}

NodeClient.prototype.sendBlock = function sendBlock(requestMessage, metadata, callback) {
  if (arguments.length === 2) {
    callback = arguments[1];
  }
  var client = grpc.unary(Node.SendBlock, {
    request: requestMessage,
    host: this.serviceHost,
    metadata: metadata,
    transport: this.options.transport,
    debug: this.options.debug,
    onEnd: function (response) {
      if (callback) {
        if (response.status !== grpc.Code.OK) {
          var err = new Error(response.statusMessage);
          err.code = response.status;
          err.metadata = response.trailers;
          callback(err, null);
        } else {
          callback(null, response.message);
        }
      }
    }
  });
  return {
    cancel: function () {
      callback = null;
      client.close();
    }
  };
};

NodeClient.prototype.sendTransaction = function sendTransaction(requestMessage, metadata, callback) {
  if (arguments.length === 2) {
    callback = arguments[1];
  }
  var client = grpc.unary(Node.SendTransaction, {
    request: requestMessage,
    host: this.serviceHost,
    metadata: metadata,
    transport: this.options.transport,
    debug: this.options.debug,
    onEnd: function (response) {
      if (callback) {
        if (response.status !== grpc.Code.OK) {
          var err = new Error(response.statusMessage);
          err.code = response.status;
          err.metadata = response.trailers;
          callback(err, null);
        } else {
          callback(null, response.message);
        }
      }
    }
  });
  return {
    cancel: function () {
      callback = null;
      client.close();
    }
  };
};

NodeClient.prototype.getHistory = function getHistory(requestMessage, metadata, callback) {
  if (arguments.length === 2) {
    callback = arguments[1];
  }
  var client = grpc.unary(Node.GetHistory, {
    request: requestMessage,
    host: this.serviceHost,
    metadata: metadata,
    transport: this.options.transport,
    debug: this.options.debug,
    onEnd: function (response) {
      if (callback) {
        if (response.status !== grpc.Code.OK) {
          var err = new Error(response.statusMessage);
          err.code = response.status;
          err.metadata = response.trailers;
          callback(err, null);
        } else {
          callback(null, response.message);
        }
      }
    }
  });
  return {
    cancel: function () {
      callback = null;
      client.close();
    }
  };
};

NodeClient.prototype.getBalance = function getBalance(requestMessage, metadata, callback) {
  if (arguments.length === 2) {
    callback = arguments[1];
  }
  var client = grpc.unary(Node.GetBalance, {
    request: requestMessage,
    host: this.serviceHost,
    metadata: metadata,
    transport: this.options.transport,
    debug: this.options.debug,
    onEnd: function (response) {
      if (callback) {
        if (response.status !== grpc.Code.OK) {
          var err = new Error(response.statusMessage);
          err.code = response.status;
          err.metadata = response.trailers;
          callback(err, null);
        } else {
          callback(null, response.message);
        }
      }
    }
  });
  return {
    cancel: function () {
      callback = null;
      client.close();
    }
  };
};

exports.NodeClient = NodeClient;

