use std::{borrow::Borrow, ops::Deref};

#[derive(Clone)]
struct Policy{
    policy_body: String
}

impl Policy {
    fn new(policy_body: String) -> Policy{
        Policy{
            policy_body
        }
    }
}

#[derive(Clone)]
struct Consent{
    consent_body: String
}

impl Consent {
    fn new(consent_body: String) -> Consent{
        Consent{
            consent_body
        }
    }
}

struct Cert<T>{
    content: T
}

impl <T> Cert<T>{
    fn new(content: T) -> Cert<T> {
        Cert{
            content
        }
    }

    fn unwrap(&self) -> &T{
        &self.content
    }
}

struct Proof<T>{
    content: T
}

impl <T> Proof<T> {
    fn new(content: T) -> Self {
        Self{
            content
        }
    }

    fn unwrap(&self) -> &T{
        &self.content
    }
}

struct OWebsite{
    i_parent: IParent,
}

impl OWebsite{
    fn new(i_parent: IParent) -> OWebsite{
        OWebsite{
            i_parent,
        }
    }

    // Ceritfy Policy, helper method for send
    fn m_policy(policy: Policy) -> Cert<Policy>{
        Cert::new(policy)
    }

    // recives the policy from website and passes it on
    fn send(&self, policy: Policy){
        
        let cert_policy = OWebsite::m_policy(policy);
        println!("OWebsite created Cert<Policy>");
        println!("OWebsite sent Cert<Policy> to IParent");
        self.i_parent.recive(cert_policy);
        
    }
}

struct IWebsite{
    website: Website
}

impl IWebsite{
    fn new(website: Website) -> Self{
        Self{
            website
        }
    }
    fn pi_consent(cons_cert: Cert<Consent>) -> Consent{
        // Here it should by some means check the certificate

        // Gives us back a clone so it no longer references the consent inside the certificate
        // To get back the "pointer" and borrow the value remove clone
        cons_cert.unwrap().clone()
    }


}

struct Website{
    policy: Policy,
    output: OWebsite
}



impl Website {
    fn new(policy: Policy, output: OWebsite) -> Website{
        Website{
            policy,
            output: output
        }
    }

    fn start(&self) {
        println!("Website sending policy to OWebsite");
        &self.output.send(self.policy.clone());
        
    }
}



struct IParent{
    parent: Parent
}

impl IParent {
    fn new(parent: Parent) -> IParent{
        IParent{
            parent
        }
    }

    fn pi_policy(cert_policy: Cert<Policy>) -> Policy {
        cert_policy.unwrap().clone()
    }

    fn recive(&self, cert_policy: Cert<Policy>){
        println!("IParent Recived Cert<Policy>");
        let policy = IParent::pi_policy(cert_policy);
        println!("IParent unwrapped Cert<Policy> and gave it to parent");
        self.parent.give(policy.clone());
    }
}

struct OParent{
    output: IWebsite
}

impl OParent {
    fn new(output: IWebsite) -> Self {
        Self{
            output
        }
    }

    fn p_policy(policy: Policy) -> Proof<Policy>{
        Proof::new(policy)
    }

    fn m_consent(consent: Consent, policy_proof: Proof<Policy>) -> Cert<Consent>{

        // Somehow we need to bake in the proof of the policy into the certification of Consent
        // This may be done through having optional fields inside the consent that allows you to add 
        // Proofs into the data sent. This means that website will have access to the proofs if it later
        // needs to prove that parent has sent it(non-repudiation)

        Cert::new(consent)
    }

    fn send(&self, consent: Consent, policy: Policy){
        println!("OParent Recived consent and policy");

        let policy_proof = OParent::p_policy(policy);

        let consent_cert = OParent::m_consent(consent, policy_proof);
        println!("OParent Created a Proof<Policy> and Cert<Consent> and sent it to IWebsite");
    }
}

struct Parent{
    output: OParent,
    consent: String,
}

impl Parent {
    fn new(output: OParent, consent: String) -> Self {
        Self {
            output,
            consent
        }
    }

    fn give(&self, policy: Policy){
        println!("Parent Recived Policy and sent consent to OParent");
        //self.output.send(self.consent)
    }
}

fn main(){

    let policy = Policy::new("Agree to terms and conditions".to_string());
    let o_parent = OParent::new();
    let parent = Parent::new(o_parent, "I give consent".to_string());
    let i_parent = IParent::new(parent);
    let o_website = OWebsite::new(i_parent);

    let website = Website::new(policy,o_website);

    website.start()

}