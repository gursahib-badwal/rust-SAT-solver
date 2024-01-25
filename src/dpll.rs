pub mod cnf_formula;
use cnf_formula::*;

pub fn find_propogatable(f:& Formula) -> Option<(Variable,bool)> {
    // unimplemented!()
    for current_clause in f {
        if current_clause.len() == 1 {
            match current_clause[0] { 
                Atom::Base(v) => return Some((v, true)),
                Atom::Not(v) => return Some((v, false))
            };
        };
    };
    None 
}

pub fn propogate_unit(f:& mut Formula,v:Variable,b:bool) {
    // unimplemented!()
    if b == true {
        f.retain(|clause| !contains_base_v(clause, v)); //remove all the clauses that contain any Atom::Base(v)

        for clause in f {
        clause.retain(|atom| !confirm_not_v(atom, v));
        }
    }

    else { //the vice versa when the variable v is set to false
        f.retain(|clause| !contains_not_v(clause, v));

        for clause in f {
        clause.retain(|atom| !confirm_base_v(atom, v));
        }
    }


 fn contains_base_v(clause: &Clause, v: Variable) -> bool {
    for atom in clause {
        if confirm_base_v(atom, v) {
            return true
        }
    }
    return false
 }

 fn contains_not_v(clause: &Clause, v: Variable) -> bool {
    for atom in clause {
        if confirm_not_v(atom, v) {
            return true
        }
    }
    return false
 }

 fn confirm_base_v(element: &Atom, v: Variable) -> bool {
    match element {
        Atom::Base(variable) if *variable == v => true,
        _ => false, 
    }
 }

 fn confirm_not_v(element: &Atom, v: Variable) -> bool {
    match element {
        Atom::Not(variable) if *variable == v => true,
        _ => false, 
    }
 }

}



pub fn find_pure_var(f:& Formula) -> Option<Variable> {
    // unimplemented!()
    let unique_variables: Vec<Variable> = get_vars(f);

    for var in unique_variables {
        if is_pure(f, var) == true {
            return Some(var);
        }
    }

    return None
}

pub fn assign_pure_var(f: & mut Formula, v: Variable) {
    // unimplemented!()
    f.retain(|clause| !has_var_clause(clause, v));
}

pub fn unit_propogate(f:& mut Formula) {
    match find_propogatable(f) {
        Option::None => return,
        Option::Some((v,b)) => {
            propogate_unit(f, v, b);
            unit_propogate(f)
        }
    }
}

pub fn assign_pure_vars(f:& mut Formula) {
    match find_pure_var(f) {
        Option::None => return,
        Option::Some(v) => {
            assign_pure_var(f,v);
            assign_pure_vars(f); 
        }
    }
}

pub fn dpll(f:& mut Formula) -> bool {
    // unimplemented!()
    unit_propogate(f);
    assign_pure_vars(f);
    if f.is_empty() == true {
        return true
    }

    for clause in f.clone() { //had to f.clone() here because f is mutable and therefore iterating over it will move it therefore f will be lost
        if clause.is_empty() == true {
            return false
        }
    }

    let chosen_variable = get_vars(f)[0]; //this piece of code gets disrupted if f is lost above

    let mut f_copy = f.clone();

    f_copy.push(vec![Atom::Base(chosen_variable)]);
    return dpll(&mut f_copy);
}