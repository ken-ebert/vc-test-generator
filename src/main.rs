use std::io;
use std::io::Read;
use std::env;
use std::fs;

use serde_json::json;
use serde::{Serialize, Deserialize};
use serde_json::Value::Null;
use serde_json::Value::Bool;
use serde_json::Value::Number;
use serde_json::Value::String as JString;
use serde_json::Value::Array;
use serde_json::Value::Object;

use regex::Regex;

//use utils::cstring::CStringUtils;
use std::ffi::CString;

use rust_libindy_wrapper::native::Handle;
use rust_libindy_wrapper::native::wallet;
use rust_libindy_wrapper::wallet::Wallet;

fn main()  {
    //println!("Create a Verifiable Credential!");

    //println!("Initialize");

	//println!("Process Command Line Args");
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    //println!("Input file = {}", filename);

/* 
    //println!("Issuer");
    let issuer_did = "SXNzdWVy";
    let issuer_wallet_config = r#"{"id": "issuer_wallet"}"#;
    let issuer_wallet_credentials = r#"{"key": "issuer_wallet_key"}"#;
    
    //println!("Holder");
    let holder_did = "SG9sZGVy";
    let holder_wallet_config = r#"{"id": "holder_wallet"}"#;
    let holder_wallet_credentials = r#"{"key": "holder_wallet_key"}"#;
   
    //println!("Wallets");
    
	//println!("Create Issuer Wallet");
    match Wallet::create(issuer_wallet_config, issuer_wallet_credentials) {
    	Ok(e) => {},//println!("Create issuer wallet, e = {:?}", e),
    	Err(f) => {
    		//println!("Error creating issuer wallet, {:?}",f);
    		Wallet::delete(issuer_wallet_config, issuer_wallet_credentials).unwrap();
    		//println!("Success deleting issuer wallet");
    		match Wallet::create(issuer_wallet_config , issuer_wallet_credentials) {
    			Ok(g) => {},
    			//println!("Success creating issuer wallet, g = {:?}", g),
    			Err(h) => {
    				println!("Error on retry to create issuer wallet, {:?}",h);
    				return; // too many errors, quit
    			}
    		}
    	}
    };

	//println!("Open Issuer Wallet");
	let issuer_wallet_handle = Wallet::open(issuer_wallet_config,issuer_wallet_credentials).unwrap();

	//println!("Create Holder Wallet");
    match Wallet::create(holder_wallet_config, holder_wallet_credentials) {
    	Ok(e) => {}, //println!("Create holder wallet, e = {:?}", e),
    	Err(f) => {
    		//println!("Error creating holder wallet, {:?}",f);
    		Wallet::delete(holder_wallet_config, holder_wallet_credentials).unwrap();
    		//println!("Success deleting holder wallet");
    		match Wallet::create(holder_wallet_config , holder_wallet_credentials) {
    			Ok(g) => {},//println!("Success creating holder wallet, g = {:?}", g),
    			Err(h) => {
    				println!("Error on retry to create holder wallet, {:?}",h);
    				return; // too many errors, quit
    			}
    		}
    	}
    }
	

	//println!("Open Holder Wallet");
	let holder_wallet_handle = Wallet::open(holder_wallet_config,holder_wallet_credentials).unwrap();
	//dbg!(&holder_wallet_credentials);
*/

	//println!("Schema");
	let schema_name = "VC_test";
	let schema_version = "1.0";
	//println!("Read the Sample Credential");
	let sample_credential: String = fs::read_to_string(filename)
		.expect("Error reading the sample credential.");
	// Read from stdin - no longer needed for the test suite
	//let mut sample_credential = String::new();
	//io::stdin().read_to_string(&mut sample_credential)
	//	.expect("Error reading the sample credential.");
		
	//println!("Sample credential:\n{}", sample_credential);

    //println!("Convert the Sample Credential to json");
    //let schema_json = serde_json::json!(&sample_credential);

    let schema_json: RawSchema = serde_json::from_str(&sample_credential).unwrap();
    //println!("The context is {:?}", schema_json.context);
    //println!("The id is {}", schema_json.id);
    //println!("The type is {:?}", schema_json.credential_type);
	
	//println!("Output Verifiable Credential");
	//println!("Using serde_json::to_string");
	//println!("{:?}", serde_json::to_string(&schema_json).unwrap());
	//println!("I'll do it myself");

	// Start the credential
	println!("{{");


	// Process context
	// is the context big enough? (at least 2 values)
	if schema_json.context.len() < 1 {
		panic!("Context empty."); // quit function std::process::exit(Err_num)
	}
	if schema_json.context.len() < 2 {
		panic!("Context only has one value.");
	}
	// Start the context
	println!("  \"@context\": [");
	let mut need_context_comma = false;
	//let mut index = 0;
	// for each context in the context vector of Value
	//println!("Context len == {}", schema_json.context.len());
	for context_value in &schema_json.context {
		//println!("index = {}", index);
		//context_value_copy = context_value;
		match context_value {
			Null => {
				println!("Null");
				panic!("Empty context");
			},
			Bool(b) => {
				println!("Bool: {}", b);
				panic!("Bool in context");
			},
			Number(n) => {
				println!("Number: {}", n);
				panic!("Number in context");
			},
	    	serde_json::value::Value::String(context_s) => {
	    		// We found a string!
	    		if need_context_comma {
					println!(",");
				} else {
	    			// the first context must be "https://www.w3.org/2018/credentials/v1"
					if context_s != "https://www.w3.org/2018/credentials/v1" {
						// die
						panic!("The first context must be \"https://www.w3.org/2018/credentials/v1\"");
					}
				}
				print!("    \"{}\"", context_s);
			},
    		Array(a) => {
				println!("Array: Not printable");
				panic!("Array in context");
			},
	    	Object(context_object) => {
	    		// We found an object	    	
	    		if need_context_comma {
					println!(",");
				} else {
	    			// the first context must be "https://www.w3.org/2018/credentials/v1"
					// die
					panic!("The first context must be \"https://www.w3.org/2018/credentials/v1\"");
				}
				//print!("    \"{:?}\"", serde_json::to_string_pretty(&context_object).unwrap());
				print!("    {}", context_value.to_string());
			}
		}
		need_context_comma = true;
		//index += 1;
	}
	// End the context section
	println!("");
	println!("  ],");


	// id
	println!("  \"id\": \"{}\",", schema_json.id);
	// we die on input parsing if this is not a string.


	// types
	// start the types section
	println!("  \"type\": [");
	// is the context big enough? (at least 2 values)
	if schema_json.credential_type.len() < 1 {
		panic!("Type empty.");
	}
	if schema_json.credential_type.len() < 2 {
		panic!("Need at least 2 types.");
	}
	let mut need_type_comma = false;
    for type_string in schema_json.credential_type {
    	if need_type_comma {
    		// only print commas in front of 2nd to nth object
    		println!(",");
    	} else {
    		// first type
    		if type_string != "VerifiableCredential" {
				panic!("Must include \"VerifiableCredential\" as type.");
			}
    	}
	    print!("    ");
    	print!("\"{}\"", type_string);
	    need_type_comma = true;
    }
	// end the types section
	println!("");
	println!("  ],");


	// issuer
	// is issuer an URI?
	// word://string/ 
	// /\w+:(\/?\/?)[^\s]+/
	//let re = Regex::new(r"(?x)(?P<year>\d{4})-(?P<month>\d{2})-(?P<day>\d{2})").unwrap(); 
	//let caps = re.captures("2010-03-14").unwrap();
	//println!("{}", &caps["year"]);
	let re = Regex::new(r"(?P<year>\w+):(?P<slashes>/?/?)(?P<site>[^\s]+/)").unwrap(); 
	let caps = re.captures(&schema_json.issuer).unwrap();
	// FIXME capture the error and handle it.
	//println!("{}", &caps["year"]);
	//println!("{}", &caps["slashes"]);
	//println!("{}", &caps["site"]);
	println!("  \"issuer\": \"{}\",", schema_json.issuer);
	// we die on input parsing if this is not a string.

	// issuance date
	//2010-01-01T19:23:24Z
	let issuance_date_re = Regex::new(r"(?P<year>\d{4})-(?P<month>\d{2})-(?P<day>\d{2})T(?P<hour>\d{2}):(?P<minute>\d{2}):(?P<second>\d{2})Z").unwrap(); 
	let issuance_date_caps = issuance_date_re.captures(&schema_json.issuance_date).unwrap();
	println!("  \"issuanceDate\": \"{}\",", schema_json.issuance_date);

	// expiration date
	if schema_json.expiration_date != "" {
		let expiration_date_re = Regex::new(r"(?P<year>\d{4})-(?P<month>\d{2})-(?P<day>\d{2})T(?P<hour>\d{2}):(?P<minute>\d{2}):(?P<second>\d{2})Z").unwrap(); 
		let expiration_date_caps = expiration_date_re.captures(&schema_json.expiration_date).unwrap();
		println!("  \"expirationDate\": \"{}\",", schema_json.expiration_date);
	}

	// credential status
	//  "credentialStatus": {
    //		"id": "https://example.edu/status/24",
    //		"type": "CredentialStatusList2017"
 	//	},
	if schema_json.credential_status != Null {
		println!("  \"credentialStatus\": {{");
		if schema_json.credential_status["type"] != Null {
			println!("    \"type\": {},", schema_json.credential_status["type"]);
		} else {
			panic!("\"credentialStatus\" must include a type.");
		}
		if schema_json.credential_status["id"] != Null {
			println!("    \"id\": {}", schema_json.credential_status["id"]);
		} else {
			panic!("\"credentialStatus\" must include an id.");
		}
		println!("  }},");		
	}


// credential schema
	//  "credentialSchema": {
    //		"id": "https://example.org/examples/degree.json",
    //		"type": "JsonSchemaValidator2018"
	//  },
	if schema_json.credential_schema != Null {
		// start credentialSchema
		print!("  \"credentialSchema\": ");
		// Do we have more than one?
		// should either be one object or an array of objects.
		match schema_json.credential_schema {
			Null => panic!("credentialSchema is Null."),
			Bool(b) => panic!("credentialSchema is Bool \"{}\"", b),
			Number(n) => panic!("credentialSchema is Number \"{}\"", n),
    		serde_json::value::Value::String(s) => panic!("credentialSchema is string \"{}\"", s),
    		// FIXME process each array element
    		Array(_) => println!("    {},", schema_json.credential_schema.to_string()),
    		Object(m) => {
				println!("{{");
				if m["type"] != Null {
					println!("    \"type\": {},", m["type"]);
				} else {
					panic!("\"credentialSchema\" must include a type.");
				}
				if m["id"] != Null {
					println!("    \"id\": {}", m["id"]);
				} else {
					panic!("\"credentialSchema\" must include an id.");
				}
				println!("  }},"); 
    		}
    	}			
	}


	// credential subject
	// start credentialSubject
	println!("  \"credentialSubject\": ");
	match schema_json.credential_subject {
		Null => println!(" \"{{}}\""),
		Bool(b) => println!(" \"{}\"", b),
		Number(n) => println!(" \"{}\"", n),
    	serde_json::value::Value::String(s) => println!(" \"{}\"", s),
    	Array(_) => println!("    {}", schema_json.credential_subject.to_string()),
    	Object(_m) => {
			println!("  {{");
			let mut need_co_comma = false;
    		for k in _m.keys() {
    			if need_co_comma {
    				// only print commas in front of 2nd to nth object
    				println!(",");
    			}
	    		print!("    ");
    			print!("\"{}\": ", k);
	    		print!("{}", _m[k]);
	    		need_co_comma = true;
    		}
    		println!();
			print!("  }}");
    	}
	}
	println!(",");


	// proof
	// start proof
	println!("  \"proof\": ");
	match schema_json.proof {
		Null => println!("  {{ }}"),
		Bool(b) => println!(" \"{}\"", b),
		Number(n) => println!(" \"{}\"", n),
    	serde_json::value::Value::String(s) => println!(" \"{}\"", s),
    	Array(_) => println!("    {}", schema_json.proof.to_string()),
    	Object(m) => {
    		if m["type"] == Null {
    			panic!("proof must have a type.");
    		}
			println!("  {{");
			let mut need_proof_comma = false;
    		for k in m.keys() {
    			if need_proof_comma {
    				// only print commas in front of 2nd to nth object
    				println!(",");
    			}
	    		print!("    ");
    			print!("\"{}\": ", k);
	    		print!("{}", m[k]);
	    		need_proof_comma = true;
    		}
    		println!();
			println!("  }}");
    	}
	}
	// end proof
	println!();


	// end credential
	println!("}}");
  
/* 
    //println!("Cleanup");
    //println!("Close Issuer Wallet");
	match Wallet::close(issuer_wallet_handle) {
    	Ok(v) => {},//println!("Success closing issuer wallet, v = {:?}", v),
    	Err(w) => {
    		println!("Error closing issuer wallet, {:?}", w);
    	}
    }

    //println!("Close Holder Wallet");
    match Wallet::close(holder_wallet_handle) {
    	Ok(x) => {},//println!("Success closing holder wallet, x = {:?}", x),
    	Err(y) => {
    		println!("Error closing holder wallet, {:?}", y);
    	}
    }

    //println!("Delete Issuer Wallet");
    match Wallet::delete(issuer_wallet_config, issuer_wallet_credentials) {
    	Ok(vi) => {},//println!("Success deleting issuer wallet, vi = {:?}", vi),
    	Err(wi) => {
    		println!("Error deleting issuer wallet, wi = {:?}", wi);
    	}
    }

    //println!("Delete Holder Wallet");
    match Wallet::delete(holder_wallet_config, holder_wallet_credentials) {
    	Ok(vh) => {},//println!("Success deleting holder wallet, vh = {:?}", vh),
    	Err(wh) => {
    		println!("Error deleting holder wallet, wh = {:?}", wh);
    	}
    }

    // Wait before exit
    //let mut a_line_of_text = String::new();

    //io::stdin().read_line(&mut a_line_of_text)
    //    .expect("Failed to read line");
    //println!("You entered: {}", a_line_of_text);
*/
}

pub fn missing_expiration_date() -> String {
	return String::new();
}

pub fn missing_credential_status() -> serde_json::Value {
	return json!(null);
}

pub fn missing_credential_schema() -> serde_json::Value {
	return json!(null);
}


#[derive(Debug, Serialize, Deserialize)]
struct RawSchema {
	#[serde(rename = "@context")]
	context: Vec<serde_json::Value>,
	id: String,
	#[serde(rename = "type")]
	credential_type: Vec<String>,
	issuer: String,
	#[serde(rename = "issuanceDate")]
	issuance_date: String,
	#[serde(rename = "expirationDate")]
	#[serde(default = "missing_expiration_date")]
	expiration_date: String,
	#[serde(rename = "credentialSubject")]
	credential_subject: serde_json::Value,
	#[serde(rename = "credentialStatus")]
	#[serde(default = "missing_credential_status")]
	credential_status: serde_json::Value,
	#[serde(rename = "credentialSchema")]
	#[serde(default = "missing_credential_schema")]
	credential_schema:  serde_json::Value,
	proof: serde_json::Value
}


// temporarily borrowed from libindy/vcx/libvcx/src/utils/cstring.rs
pub fn string_to_cstring(s: String) -> CString {
    CString::new(s).unwrap()
}
