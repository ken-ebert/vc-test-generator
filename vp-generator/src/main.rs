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

use CredentialType::SingleType;
use CredentialType::MultiType;

/*
  "@context": [
    "https://www.w3.org/2018/credentials/v1",
    "https://www.w3.org/2018/credentials/examples/v1"
  ],
  "id": "urn:uuid:3978344f-8596-4c3a-a978-8fcaba3903c5",
  "type": ["VerifiablePresentation", "CredentialManagerPresentation"],
  "verifiableCredential": [{
    "id": "http://example.edu/credentials/3732",
    "type": ["VerifiableCredential", "UniversityDegreeCredential"],
    "issuer": "https://example.edu/issuers/14",
    "issuanceDate": "2010-01-01T19:23:24Z",
    "credentialSubject": {
      "id": "did:example:ebfeb1f712ebc6f1c276e12ec21",
      "degree": {
        "type": "BachelorDegree",
        "name": "<span lang='fr-CA'>Baccalauréat en musiques numériques</span>"
      }
    },
    "proof": [{
      "type": "example"
    }]
  }],
  "proof": [{
    "type": "example"
  }]
}
*/

fn main()  {
    //println!("Create a Verifiable Presentation!");


    //println!("Initialize");

	//println!("Process Command Line Args");
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    //println!("Input file = {}", filename);



	//println!("Schema");
	//let schema_name = "VC_test";
	//let schema_version = "1.0";
	//println!("Read the Sample Presentation");
	let sample_presentation: String = fs::read_to_string(filename)
		.expect("Error reading the sample Presentation.");
		
	//println!("Sample credential:\n{}", sample_credential);

    //println!("Convert the sample presentation to json");
    //let schema_json = serde_json::json!(&sample_credential);

    let mut presentation_json: RawPresentation = serde_json::from_str(&sample_presentation).unwrap();
    //println!("Extract the verifiable_credential");
    let schema_json: &RawSchema = &presentation_json.verifiable_credential[0];
    //println!("The context is {:?}", schema_json.context);
    //println!("The id is {}", schema_json.id);
    //println!("The type is {:?}", schema_json.credential_type);
	
	//println!("Output Verifiable Credential");
	//println!("Using serde_json::to_string");
	//println!("{:?}", serde_json::to_string(&schema_json).unwrap());
	//println!("I'll do it myself");

	// Start the presentation
	println!("{{");

	// Process presentation context
	// is the context big enough? (at least 2 values)
	if presentation_json.presentation_context.len() < 1 {
		panic!("Presentation context empty."); // quit function std::process::exit(Err_num)
	}
	if presentation_json.presentation_context.len() < 2 {
		panic!("Presentation context only has one value.");
	}
	// Start the context
	println!("  \"@context\": [");
	let mut need_presentation_context_comma = false;
	//let mut index = 0;
	// for each context in the context vector of Value
	//println!("Context len == {}", schema_json.context.len());
	for presentation_context_value in &presentation_json.presentation_context {
		//println!("index = {}", index);
		//context_value_copy = context_value;
		match presentation_context_value {
			Null => {
				println!("Null");
				panic!("Empty presentation context");
			},
			Bool(b) => {
				println!("Bool: {}", b);
				panic!("Bool in presentation context");
			},
			Number(n) => {
				println!("Number: {}", n);
				panic!("Number in presentation context");
			},
	    	serde_json::value::Value::String(presentation_context_s) => {
	    		// We found a string!
	    		if need_presentation_context_comma {
					println!(",");
				} else {
	    			// the first context must be "https://www.w3.org/2018/credentials/v1"
					if presentation_context_s != "https://www.w3.org/2018/credentials/v1" {
						// die
						panic!("The first presentation context must be \"https://www.w3.org/2018/credentials/v1\"");
					}
				}
				print!("    \"{}\"", presentation_context_s);
			},
    		Array(a) => {
				println!("Array: Not printable");
				panic!("Array in context");
			},
	    	Object(presentation_context_object) => {
	    		// We found an object	    	
	    		if need_presentation_context_comma {
					println!(",");
				} else {
	    			// the first context must be "https://www.w3.org/2018/credentials/v1"
					// die
					panic!("The first presentation context must be \"https://www.w3.org/2018/credentials/v1\"");
				}
				//print!("    \"{:?}\"", serde_json::to_string_pretty(&context_object).unwrap());
				print!("    {}", presentation_context_value.to_string());
			}
		}
		need_presentation_context_comma = true;
		//index += 1;
	}
	// End the presentation context section
	println!("");
	println!("  ],");


	// presentation id
	// If present, print it
	if presentation_json.presentation_id != "" {
		println!("  \"id\": \"{}\",", presentation_json.presentation_id);
		// we die on input parsing if this is not a string.
	}


	// presentation types
	// start the types section
	println!("  \"type\": [");
	match presentation_json.presentation_type {
		Null => {
			println!("Null");
			panic!("Empty presentation_type");
		},
		Bool(b) => {
			println!("Bool: {}", b);
			panic!("Bool in presentation_type");
		},
		Number(n) => {
			println!("Number: {}", n);
			panic!("Number in presentation_type");
		},
	    serde_json::value::Value::String(presentation_type_s) => {
	    	// We found a string!
	    	// first type
    		if presentation_type_s != "VerifiablePresentation" {
				panic!("Must include \"VerifiablePresentation\" as type.");
			}
			print!("    \"{}\"", presentation_type_s);
		},
    	Array(presentation_type_array) => {
			// is the presentationType big enough? (at least 1 value)
			if presentation_type_array.len() < 1 {
				panic!("Presentation type empty.");
			}
			let mut need_presentation_type_comma = false;
    		for presentation_type_string in presentation_type_array {
    			if need_presentation_type_comma {
    				// only print commas in front of 2nd to nth object
    				println!(",");
    			} else {
    				// first type
    				if presentation_type_string != "VerifiablePresentation" {
						panic!("Must include \"VerifiablePresentation\" as first type.");
					}
    			}
    			print!("    {}", presentation_type_string);
	    		need_presentation_type_comma = true;
    		}
    		println!("");
		},
	    Object(_o) => {
	    	// We found an object	    	
			panic!("Must include \"VerifiablePresentation\" as type, not an object.");
		}

/*	    SingleType(presentation_type_s) => {
	    	// first type
    		if presentation_type_s != "VerifiablePresentation" {
				panic!("Must include \"VerifiablePresentation\" as type.");
			}
			print!("    \"{}\"", presentation_type_s);
		},
	   	MultiType(pa) => {
			// is the presentationType big enough? (at least 1 value)
			if pa.len() < 1 {
				panic!("Presentation type empty.");
			}
			let mut need_presentation_type_comma = false;
    		for presentation_type_string in pa {
    			if need_presentation_type_comma {
    				// only print commas in front of 2nd to nth object
    				println!(",");
    			} else {
    				// first type
    				if presentation_type_string != "VerifiablePresentation" {
						panic!("Must include \"VerifiablePresentation\" as first type.");
					}
    			}
	    		print!("    ");
    			print!("\"{}\"", presentation_type_string);
	    		need_presentation_type_comma = true;
    		}
    		println!("");
		},*/
	};
	// end the types section
	println!("  ],");


	// start the embedded credentials
	println!("  \"verifiableCredential\": [");


	// Start the credential
	// ******************************************************************************************************
	println!("    {{");


	// Process context
	// is the context big enough? (at least 2 values)
	if schema_json.context.len() < 1 {
		panic!("Context empty."); // quit function std::process::exit(Err_num)
	}
	if schema_json.context.len() < 2 {
		panic!("Context only has one value.");
	}
	// Start the context
	println!("    \"@context\": [");
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
				print!("      \"{}\"", context_s);
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
				print!("      {}", context_value.to_string());
			}
		}
		need_context_comma = true;
		//index += 1;
	}
	// End the context section
	println!("");
	println!("    ],");


	// If present, print it
	if schema_json.id != "" {
		println!("  \"id\": \"{}\",", schema_json.id);
		// we die on input parsing if this is not a string.
	}


	// types
	// start the types section
	println!("    \"type\": [");
	// is the context big enough? (at least 2 values)
	if schema_json.credential_type.len() < 1 {
		panic!("Type empty.");
	}
	if schema_json.credential_type.len() < 2 {
		panic!("Need at least 2 types.");
	}
	let mut need_type_comma = false;
    for type_string in &schema_json.credential_type {
    	if need_type_comma {
    		// only print commas in front of 2nd to nth object
    		println!(",");
    	} else {
    		// first type
    		if type_string != "VerifiableCredential" {
				panic!("Must include \"VerifiableCredential\" as type.");
			}
    	}
	    print!("      ");
    	print!("\"{}\"", type_string);
	    need_type_comma = true;
    }
	// end the types section
	println!("");
	println!("    ],");


	// issuer
	// is issuer an URI?
	// word://string/ 
	// /\w+:(\/?\/?)[^\s]+/
	//let re = Regex::new(r"(?x)(?P<year>\d{4})-(?P<month>\d{2})-(?P<day>\d{2})").unwrap(); 
	//let caps = re.captures("2010-03-14").unwrap();
	//println!("{}", &caps["year"]);
	let re = Regex::new(r"(?P<scheme>\w+):(?:(?:(?P<url>//[.\w]+)(?:(/(?P<path>[/\w]+)?)?))|(?:(?P<method>\w+):(?P<id>\w+)))").unwrap(); 
	let caps = re.captures(&schema_json.issuer).unwrap();
	//let re = Regex::new(r"(?P<year>\w+):(?P<slashes>/?/?)(?P<site>[^\s]+/)").unwrap(); 
	//let caps = re.captures(&schema_json.issuer).unwrap();
	// FIXME capture the error and handle it.
	//println!("{}", &caps["year"]);
	//println!("{}", &caps["slashes"]);
	//println!("{}", &caps["site"]);
	println!("    \"issuer\": \"{}\",", schema_json.issuer);
	// we die on input parsing if this is not a string.


	// issuance date
	//2010-01-01T19:23:24Z
	if schema_json.issuance_date == "" {
		panic!("Verifiable Credentials must contain an issuanceDate.");
	} else {
		let issuance_date_re = Regex::new(r"(?P<year>\d{4})-(?P<month>\d{2})-(?P<day>\d{2})T(?P<hour>\d{2}):(?P<minute>\d{2}):(?P<second>\d{2})Z").unwrap(); 
		let issuance_date_caps = issuance_date_re.captures(&schema_json.issuance_date).unwrap();
		println!("    \"issuanceDate\": \"{}\",", schema_json.issuance_date);
	}


	// expiration date
	if schema_json.expiration_date != "" {
		let expiration_date_re = Regex::new(r"(?P<year>\d{4})-(?P<month>\d{2})-(?P<day>\d{2})T(?P<hour>\d{2}):(?P<minute>\d{2}):(?P<second>\d{2})Z").unwrap(); 
		let expiration_date_caps = expiration_date_re.captures(&schema_json.expiration_date).unwrap();
		println!("    \"expirationDate\": \"{}\",", schema_json.expiration_date);
	}


	// credential status
	//  "credentialStatus": {
    //		"id": "https://example.edu/status/24",
    //		"type": "CredentialStatusList2017"
 	//	},
	if schema_json.credential_status != Null {
		println!("    \"credentialStatus\": {{");
		if schema_json.credential_status["type"] != Null {
			println!("      \"type\": {},", schema_json.credential_status["type"]);
		} else {
			panic!("\"credentialStatus\" must include a type.");
		}
		if schema_json.credential_status["id"] != Null {
			println!("      \"id\": {}", schema_json.credential_status["id"]);
		} else {
			panic!("\"credentialStatus\" must include an id.");
		}
		println!("    }},");		
	}


	// credential schema
	//  "credentialSchema": {
    //		"id": "https://example.org/examples/degree.json",
    //		"type": "JsonSchemaValidator2018"
	//  },
	if schema_json.credential_schema != Null {
		// start credentialSchema
		print!("    \"credentialSchema\": ");
		// Do we have more than one?
		// should either be one object or an array of objects.
		match &schema_json.credential_schema {
			Null => panic!("credentialSchema is Null."),
			Bool(b) => panic!("credentialSchema is Bool \"{}\"", b),
			Number(n) => panic!("credentialSchema is Number \"{}\"", n),
    		serde_json::value::Value::String(s) => panic!("credentialSchema is string \"{}\"", s),
    		// FIXME process each array element
    		Array(_) => println!("      {},", schema_json.credential_schema.to_string()),
    		Object(m) => {
				println!("{{");
				if m["type"] != Null {
					println!("      \"type\": {},", m["type"]);
				} else {
					panic!("\"credentialSchema\" must include a type.");
				}
				if m["id"] != Null {
					println!("      \"id\": {}", m["id"]);
				} else {
					panic!("\"credentialSchema\" must include an id.");
				}
				println!("    }},"); 
    		}
    	}			
	}


	// credential subject
	// start credentialSubject
	println!("    \"credentialSubject\": ");
	match &schema_json.credential_subject {
		Null => println!(" \"{{}}\""),
		Bool(b) => println!(" \"{}\"", b),
		Number(n) => println!(" \"{}\"", n),
    	serde_json::value::Value::String(s) => println!(" \"{}\"", s),
    	Array(_) => println!("      {}", schema_json.credential_subject.to_string()),
    	Object(_m) => {
			println!("    {{");
			let mut need_co_comma = false;
    		for k in _m.keys() {
    			if need_co_comma {
    				// only print commas in front of 2nd to nth object
    				println!(",");
    			}
	    		print!("      ");
    			print!("\"{}\": ", k);
	    		print!("{}", _m[k]);
	    		need_co_comma = true;
    		}
    		println!();
			print!("    }}");
    	}
	}
	println!(",");

	// proof
	// start proof
	println!("    \"proof\": ");
	match &schema_json.proof {
		Null => println!("  {{ }}"),
		Bool(b) => println!(" \"{}\"", b),
		Number(n) => println!(" \"{}\"", n),
    	serde_json::value::Value::String(s) => println!(" \"{}\"", s),
    	Array(_) => println!("      {}", schema_json.proof.to_string()),
    	Object(m) => {
    		if m["type"] == Null {
    			panic!("proof must have a type.");
    		}
			println!("    {{");
			let mut need_proof_comma = false;
    		for k in m.keys() {
    			if need_proof_comma {
    				// only print commas in front of 2nd to nth object
    				println!(",");
    			}
	    		print!("      ");
    			print!("\"{}\": ", k);
	    		print!("{}", m[k]);
	    		need_proof_comma = true;
    		}
    		println!();
			println!("    }}");
    	}
	}
	// end proof
	//println!();

	// end credential
	println!("    }}");

	// end verifiableCredential in presentation
	println!("  ],");

	// presentation proof
	// start proof
	println!("  \"proof\": ");
	match &presentation_json.presentation_proof {
		Null => println!("  {{ }}"),
		Bool(b) => println!(" \"{}\"", b),
		Number(n) => println!(" \"{}\"", n),
    	serde_json::value::Value::String(s) => println!(" \"{}\"", s),
    	Array(_) => println!("    {}", presentation_json.presentation_proof.to_string()),
    	Object(m) => {
    		if m["type"] == Null {
    			panic!("Presentation proof must have a type.");
    		}
			println!("  {{");
			let mut need_presentation_proof_comma = false;
    		for k in m.keys() {
    			if need_presentation_proof_comma {
    				// only print commas in front of 2nd to nth object
    				println!(",");
    			}
	    		print!("    ");
    			print!("\"{}\": ", k);
	    		print!("{}", m[k]);
	    		need_presentation_proof_comma = true;
    		}
    		println!();
			println!("  }}");
    	}
	}
	// end presentation proof
	//println!();

	// end presentation
	println!("  }}");



  
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

#[derive(Serialize, Deserialize)]
pub enum CredentialType {
      SingleType(String),
      MultiType(Vec<String>)
}

pub fn missing_presentation_id() -> String {
	return String::new();
}

pub fn missing_credential_id() -> String {
	return String::new();
}

pub fn missing_credential_issuance_date() -> String {
	return String::new();
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


//#[derive(Debug, Serialize, Deserialize)]
#[derive(Serialize, Deserialize)]
struct RawPresentation {
	#[serde(rename = "@context")]
	presentation_context: Vec<serde_json::Value>,
	#[serde(rename = "id")]
	#[serde(default = "missing_presentation_id")]
	presentation_id: String,
	//#[serde(rename = "type")]
	//presentation_type: CredentialType,
	#[serde(rename = "type")]
	presentation_type: serde_json::Value,
	#[serde(rename = "verifiableCredential")]
	verifiable_credential: Vec<RawSchema>,
	#[serde(rename = "proof")]
	presentation_proof: serde_json::Value
}

#[derive(Debug, Serialize, Deserialize)]
struct RawSchema {
	#[serde(rename = "@context")]
	context: Vec<serde_json::Value>,
	#[serde(default = "missing_credential_id")]
	id: String,
	#[serde(rename = "type")]
	credential_type: Vec<String>,
	issuer: String,
	#[serde(rename = "issuanceDate")]
	#[serde(default = "missing_credential_issuance_date")]
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
