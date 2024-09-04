
use std::collections::HashMap;
use std::io;
use std::sync::Mutex;
use std::rc::Rc;
use std::io::Write;
use lazy_static::lazy_static;

lazy_static! {
    static ref MEM_LOC: Mutex<HashMap<String, i32>> = Mutex::new(HashMap::new());
    static ref COUNTER: Mutex<i32> = Mutex::new(1);
}

pub struct NumberNode {
    val: i32,
}

impl NumberNode {
    pub fn new(val: i32) -> Self {
        NumberNode { val }
    }

    pub fn generate_code(&self, out: &mut dyn Write) -> io::Result<()> {
        writeln!(out, "ldi A {}", self.val)?;
        println!("ldi A {}", self.val);
        println!("generating in b number node");
        Ok(())
    }

    pub fn generate_l(&self, out: &mut dyn Write) -> io::Result<()> {
        writeln!(out, "ldi A {}", self.val)?;
        println!("ldi A {}", self.val);
        println!("generating in l number node");
        Ok(())
    }

    pub fn generate_r(&self, out: &mut dyn Write) -> io::Result<()> {
        writeln!(out, "ldi B {}", self.val)?;
        println!("ldi B {}", self.val);
        println!("generating in r number node");
        Ok(())
    }
}

pub struct VariableNode {
    name: String,
    id: i32,
}

impl VariableNode {
    pub fn new(name: String) -> Self {
        let mut mem_loc = MEM_LOC.lock().unwrap();
        let mut counter = COUNTER.lock().unwrap();

        if !mem_loc.contains_key(&name) {
            mem_loc.insert(name.clone(), *counter);
            *counter += 1;
        }

        let id = mem_loc[&name];
        VariableNode { name, id }
    }

    pub fn generate_code(&self, out: &mut dyn Write) -> io::Result<()> {
        println!("generating in b VariableNode");
        writeln!(out, "sta {}", self.id)?;
        Ok(())
    }

    pub fn generate_l(&self, out: &mut dyn Write) -> io::Result<()> {
        println!("in variable node L for variable {}", self.name);
        writeln!(out, "lda {}", self.id)?;
        Ok(())
    }

    pub fn generate_r(&self, out: &mut dyn Write) -> io::Result<()> {
        println!("in variable node R");
        writeln!(out, "mov B M {}", self.id)?;
        Ok(())
    }
    
    pub fn is_declared(name: &str) -> bool {
        MEM_LOC.lock().unwrap().contains_key(name)
    }
}

pub struct BinaryOpNode {
    l: Rc<dyn ASTNode>,
    op: String,
    r: Rc<dyn ASTNode>,
}

impl BinaryOpNode {
    pub fn new(l: Rc<dyn ASTNode>, op: String, r: Rc<dyn ASTNode>) -> Self {
        BinaryOpNode { l, op, r }
    }

    pub fn generate_l(&self, out: &mut dyn Write) -> io::Result<()> {
        self.generate_code(out).expect("error in binary node");
        Ok(())
    }

    pub fn generate_r(&self, out: &mut dyn Write) -> io::Result<()> {
        self.generate_code(out).expect("error in binary node");
        Ok(())
    }


    pub fn generate_code(&self, out: &mut dyn Write) -> io::Result<()> {
        println!("in binary node");
        self.l.generate_l(out)?;
        //print what the type of r is 
        self.r.generate_r(out)?;
        match self.op.as_str() {

            "+" =>{println!("Adding");
                  writeln!(out, "add")?},
            "-" => writeln!(out, "sub")?,
            "==" => writeln!(out, "cmp")?,
            _ => return Err(io::Error::new(io::ErrorKind::InvalidInput, "unsupported binary op")),
        };
        Ok(())
    }
}

pub struct VariableDec {
    var_name: String,
    val: Rc<dyn ASTNode>,
}

impl VariableDec {
    pub fn new(var_name: String, val: Rc<dyn ASTNode>) -> Self {
        let mut mem_loc = MEM_LOC.lock().unwrap();
        if !mem_loc.contains_key(&var_name) {
            let mut counter = COUNTER.lock().unwrap();
            mem_loc.insert(var_name.clone(), *counter);
            *counter += 1;
        }
        VariableDec { var_name, val }
    }

    fn get_memory_location(&self) -> i32 {
        MEM_LOC.lock().unwrap()[&self.var_name]
    }
    pub fn generate_code(&self, out: &mut dyn Write) -> io::Result<()> {
        println!("variable declaration in b");

        // Generate code for the right-hand side value (which could be a BinaryOpNode)
        self.val.generate_code(out)?;

        println!("storing value to memory location");
        writeln!(out, "sta {}", self.get_memory_location())?; // Store the result in memory for 'y'
        Ok(())
    }
}
pub struct ConditionalNode {
    cond: Rc<dyn ASTNode>,
    then_branch: Rc<dyn ASTNode>,
    else_branch: Option<Rc<dyn ASTNode>>,
}

impl ConditionalNode {
    pub fn new(cond: Rc<dyn ASTNode>, then_branch: Rc<dyn ASTNode>, else_branch: Option<Rc<dyn ASTNode>>) -> Self {
        ConditionalNode { cond, then_branch, else_branch }
    }

    pub fn generate_code(&self, out: &mut dyn Write) -> io::Result<()> {
        self.cond.generate_code(out)?;
        writeln!(out, "jnz %else_branch")?;
        println!("jnz %else_branch");

        self.then_branch.generate_code(out)?;
        writeln!(out, "jmp %endif")?;
        println!("jmp %endif");

        writeln!(out, "else_branch:")?;
        println!("else_branch:");

        if let Some(ref else_br) = self.else_branch {
            else_br.generate_code(out)?;
        }
        writeln!(out, "endif:")?;
        println!("endif:");
        Ok(())
    }
}

pub struct BlockNode {
    statements: Vec<Rc<dyn ASTNode>>,
}

impl BlockNode {
    pub fn new() -> Self {
        BlockNode {
            statements: Vec::new(),
        }
    }

    pub fn add_stat(&mut self, statement: Rc<dyn ASTNode>) {
        self.statements.push(statement);
    }

    pub fn generate_code(&self, out: &mut dyn Write) -> io::Result<()> {
        println!("started generating code");
        for statement in &self.statements {
            statement.generate_code(out)?;
        }
        println!("ended generating code");
        Ok(())
    }
}
//add trait debug

pub trait ASTNode {
    fn generate_code(&self, out: &mut dyn Write) -> io::Result<()>;
    fn generate_l(&self, _out: &mut dyn Write) -> io::Result<()> { Ok(()) }
    fn generate_r(&self, _out: &mut dyn Write) -> io::Result<()> { Ok(()) }
}


impl ASTNode for NumberNode {
    fn generate_code(&self, out: &mut dyn Write) -> io::Result<()> {
        self.generate_code(out)
    }
    fn generate_l(&self, out: &mut dyn Write) -> io::Result<()> {
        self.generate_l(out)
    }

    fn generate_r(&self, out: &mut dyn Write) -> io::Result<()> {
        self.generate_r(out)
    }

}

impl ASTNode for VariableNode {
    fn generate_code(&self, out: &mut dyn Write) -> io::Result<()> {
        self.generate_code(out)
    }

    fn generate_l(&self, out: &mut dyn Write) -> io::Result<()> {
        self.generate_l(out)
    }

    fn generate_r(&self, out: &mut dyn Write) -> io::Result<()> {
        self.generate_r(out)
    }
}

impl ASTNode for BinaryOpNode {
    fn generate_code(&self, out: &mut dyn Write) -> io::Result<()> {
        self.generate_code(out)
    }
    fn generate_l(&self, out: &mut dyn Write) -> io::Result<()> {
        self.generate_l(out)
    }

    fn generate_r(&self, out: &mut dyn Write) -> io::Result<()> {
        self.generate_r(out)
    }
}

impl ASTNode for VariableDec {
    fn generate_code(&self, out: &mut dyn Write) -> io::Result<()> {
        self.generate_code(out)
    }
}

impl ASTNode for ConditionalNode {
    fn generate_code(&self, out: &mut dyn Write) -> io::Result<()> {
        self.generate_code(out)
    }
}

impl ASTNode for BlockNode {
    fn generate_code(&self, out: &mut dyn Write) -> io::Result<()> {
        self.generate_code(out)
    }
}
