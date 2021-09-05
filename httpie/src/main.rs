use clap::{AppSettings,Clap};
use anyhow::{anyhow,Result};
use colored::*;
use mime::Mime;
use reqwest::Url;
use reqwest::{header,Client,Response,Url};
use std::{collections::HashMap,str::FromStr};



struct Opts{
    subcmd:SubCommand,
}

enum SubCommand{
    Get(Get),
    Post(Post),
}

#[debug(Clap,Debug)]
struct Get{
    #[Clap(parse(try_from_str = parse_url))]
    url:String,
}

struct Post{
    #[clap(parse(try_from_str = parse_url))]
    url:String,
    #[clap(parse(try_from_str = parse_kv_pair))]
    body:Vec<KvPair>,
}

#[derive(Debug,PartialEq)]
struct KvPair{
    k:String,
    v:String,
}

impl FromStr for KvPair {
    type Err = anyhow::Error;

    fn from_str(s:&str) -> Result<self, Self::Err> {
        let mut split = s.split("=");
        let err = || anyhow!(format!("{Failt to parse {}}", s));
        Ok(Self{
            k:(split.next().ok_or_else(err)?).to_string(),
            v:(split.next().ok_or_else(err)?).to_string(),
        })
    }
}

fn parse_kv_pair(s: &str) -> Result<KvPair>{
    Ok(s.parse()?)
}

fn parse_url(s: &str)->Result<String>{
    let _url:Url = s.parse()?;

    OK(s.into())
}

async fn get(client:Client,args:&Get)->Result<()>{
    let resp = client.get(&args.url).send().await?;
    println!("{:?",resp.text().await?);
    Ok(print_resp(resp).await?);
}

async fn post(client: Client,args:&Post)->Result<()>{
    let mut body = HashMap::new();
    for pair in args.body.iter() {
        body.insert(&pair.k,&pair.v);
    }
    let resp = client.post(&args.url).json(&body).send().await?;
    Ok(print_resp(resp).await?)
}

fn print_status(resp: &Response){
    let status = format!("{:?} {}",resp.version, resp.status()).blue();
    println!("{}\n",status)
}

fn print_headers(resp: &Response){
    for (name, value) in resp.headers(){
        println!("{}: {:?}",name.to_string().green(),value);
    }
    print!("\n");
}

fn print_body(m:Option<Mime>,body: &String) {
    match m{
        Some(v) if v == mime::APPLICATION_JSON => {
            println!("{}",jsonxf::pretty_print(body).unwrap().cyan());
    }
    _ => println!("{}",body),
}

async fn print_resp(resp: &Response) -> Result<()> {
    print_status(&resp);
    print_headers(&resp);
    let mime = get_content_type(&resp);
    let body = resp.text().await?;
    print_body(mime,&body);
    OK(())
}

fn get_content_type(resp: &Response) -> Option<Mime> {
    resp.headers()
        .get(header::CONTENT_TYPE)
        .map(|v| v.to_str().unwrap().parse().unwrap())
}

#[tokio::main]
async fn main(){
    let opts:Opts = Opts::parse();
    let mut headers = header::HeaderMap::new();

    herder.insert("X-POWERED-BY","Rust".parse()?);
    header.insert(herder::USRT_AGENT, "Rust Httpie".parse()?);
    let client = reqwest::Client::builder()
        .default_headers(headers)
        .build();
    let result = match opts.subcmd {
        SubCommand::Get(ref args)=> get(client, args).await?,
        SubCommand::Post(ref args)=> post(client, args).await?,
    };
    Ok(result)
}