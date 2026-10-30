fn main() {
    println!("=== AUDIT LEDGER SYNCHRONIZER ===");

    let mut audit_log = String::from("[Entry 1: System Boot]");

    println!("[Phase 1: Multi-Reader Inspection]");
    let r1 = &audit_log;
    let r2 = &audit_log;
    println!("Inspector A: {}", r1);
    println!("Inspector B: {}", r2);

    println!("\n[Phase 2: Single-Writer Update]");
    let writer = &mut audit_log;
    writer.push_str(" -> [Entry 2: Backup Verified]");
    println!("Lead Auditor updated log: {}", writer);

    println!("\n[Phase 3: Final Verification]");
    let final_reader = &audit_log;
    println!("Final Log State: {}", final_reader);

    println!("=================================");
}

