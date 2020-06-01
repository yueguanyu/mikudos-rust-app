// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]

const METHOD_GREETER_SERVICE_SAY_HELLO: ::grpcio::Method<super::greeter::HelloRequest, super::greeter::HelloReply> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/greeter.GreeterService/SayHello",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_GREETER_SERVICE_SAY_HI: ::grpcio::Method<super::greeter::HelloRequest, super::greeter::HelloReply> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/greeter.GreeterService/SayHi",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

#[derive(Clone)]
pub struct GreeterServiceClient {
    client: ::grpcio::Client,
}

impl GreeterServiceClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        GreeterServiceClient {
            client: ::grpcio::Client::new(channel),
        }
    }

    pub fn say_hello_opt(&self, req: &super::greeter::HelloRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::greeter::HelloReply> {
        self.client.unary_call(&METHOD_GREETER_SERVICE_SAY_HELLO, req, opt)
    }

    pub fn say_hello(&self, req: &super::greeter::HelloRequest) -> ::grpcio::Result<super::greeter::HelloReply> {
        self.say_hello_opt(req, ::grpcio::CallOption::default())
    }

    pub fn say_hello_async_opt(&self, req: &super::greeter::HelloRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::greeter::HelloReply>> {
        self.client.unary_call_async(&METHOD_GREETER_SERVICE_SAY_HELLO, req, opt)
    }

    pub fn say_hello_async(&self, req: &super::greeter::HelloRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::greeter::HelloReply>> {
        self.say_hello_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn say_hi_opt(&self, req: &super::greeter::HelloRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::greeter::HelloReply> {
        self.client.unary_call(&METHOD_GREETER_SERVICE_SAY_HI, req, opt)
    }

    pub fn say_hi(&self, req: &super::greeter::HelloRequest) -> ::grpcio::Result<super::greeter::HelloReply> {
        self.say_hi_opt(req, ::grpcio::CallOption::default())
    }

    pub fn say_hi_async_opt(&self, req: &super::greeter::HelloRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::greeter::HelloReply>> {
        self.client.unary_call_async(&METHOD_GREETER_SERVICE_SAY_HI, req, opt)
    }

    pub fn say_hi_async(&self, req: &super::greeter::HelloRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::greeter::HelloReply>> {
        self.say_hi_async_opt(req, ::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F) where F: ::futures::Future<Item = (), Error = ()> + Send + 'static {
        self.client.spawn(f)
    }
}

pub trait GreeterService {
    fn say_hello(&mut self, ctx: ::grpcio::RpcContext, req: super::greeter::HelloRequest, sink: ::grpcio::UnarySink<super::greeter::HelloReply>);
    fn say_hi(&mut self, ctx: ::grpcio::RpcContext, req: super::greeter::HelloRequest, sink: ::grpcio::UnarySink<super::greeter::HelloReply>);
}

pub fn create_greeter_service<S: GreeterService + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_GREETER_SERVICE_SAY_HELLO, move |ctx, req, resp| {
        instance.say_hello(ctx, req, resp)
    });
    let mut instance = s;
    builder = builder.add_unary_handler(&METHOD_GREETER_SERVICE_SAY_HI, move |ctx, req, resp| {
        instance.say_hi(ctx, req, resp)
    });
    builder.build()
}
