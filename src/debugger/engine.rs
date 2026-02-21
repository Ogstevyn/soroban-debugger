use crate::debugger::breakpoint::BreakpointManager;
use crate::debugger::state::DebugState;
use crate::runtime::executor::ContractExecutor;
use crate::Result;
use tracing::info;

use crate::compare::trace::{ExecutionTrace, BudgetTrace, CallEntry, EventEntry};
use std::collections::BTreeMap;

/// Core debugging engine that orchestrates execution and debugging
pub struct DebuggerEngine {
    executor: ContractExecutor,
    breakpoints: BreakpointManager,
    state: DebugState,
    paused: bool,
    last_trace: Option<ExecutionTrace>,
}

impl DebuggerEngine {
    /// Create a new debugger engine
    pub fn new(executor: ContractExecutor, initial_breakpoints: Vec<String>) -> Self {
        let mut breakpoints = BreakpointManager::new();

        // Add initial breakpoints
        for bp in initial_breakpoints {
            breakpoints.add(&bp);
            info!("Breakpoint set at function: {}", bp);
        }

        Self {
            executor,
            breakpoints,
            state: DebugState::new(),
            paused: false,
            last_trace: None,
        }
    }

    /// Execute a contract function with debugging and storage tracking
    pub fn execute(&mut self, function: &str, args: Option<&str>) -> Result<String> {
        info!("Executing function: {}", function);

        // Capture storage state before execution
        let storage_before_raw = self.executor.get_storage_snapshot()?;
        let mut storage_before = BTreeMap::new();
        for (k, v) in &storage_before_raw {
            storage_before.insert(k.clone(), v.clone());
        }

        // Check if we should break at function entry
        if self.breakpoints.should_break(function) {
            self.pause_at_function(function);
        }

        // Execute the contract
        let result = self.executor.execute(function, args)?;

        // Capture storage state after execution
        let storage_after_raw = self.executor.get_storage_snapshot()?;
        
        let mut storage_after = BTreeMap::new();
        for (k, v) in &storage_after_raw {
            storage_after.insert(k.clone(), v.clone());
        }

        // Capture budget
        let budget_info = crate::inspector::BudgetInspector::get_cpu_usage(self.executor.host());
        let budget_trace = BudgetTrace {
            cpu_instructions: budget_info.cpu_instructions,
            memory_bytes: budget_info.memory_bytes,
            cpu_limit: Some(budget_info.cpu_limit),
            memory_limit: Some(budget_info.memory_limit),
        };

        // Capture events
        let events_raw = self.executor.get_events()?;
        let events = events_raw.iter().map(|e| EventEntry {
            contract_id: e.contract_id.clone(),
            topics: e.topics.clone(),
            data: Some(e.data.clone()),
        }).collect();

        // Assemble call sequence (just the top-level call for now)
        let call_sequence = vec![CallEntry {
            function: function.to_string(),
            args: args.map(|a| a.to_string()),
            depth: 0,
            budget: Some(budget_trace.clone()),
        }];

        // Build the full trace
        let trace = ExecutionTrace {
            version: "1.0".to_string(),
            label: Some(format!("Execution of {}", function)),
            contract: Some(self.executor.contract_address().to_string()),
            function: Some(function.to_string()),
            args: args.map(|a| a.to_string()),
            storage_before,
            storage: storage_after,
            budget: Some(budget_trace),
            return_value: Some(serde_json::Value::String(result.clone())),
            call_sequence,
            events,
        };

        self.last_trace = Some(trace.clone());

        // Calculate and display storage diff if requested via some flag
        let diff = crate::inspector::StorageInspector::compute_diff(&storage_before_raw, &storage_after_raw);
        if !diff.is_empty() {
             crate::inspector::StorageInspector::display_diff(&diff);
        }

        info!("Execution completed");
        Ok(result)
    }

    /// Get the trace from the last execution
    pub fn last_trace(&self) -> Option<&ExecutionTrace> {
        self.last_trace.as_ref()
    }

    /// Step through one instruction
    pub fn step(&mut self) -> Result<()> {
        info!("Stepping...");
        self.paused = false;
        // TODO: Implement actual stepping logic
        Ok(())
    }

    /// Continue execution until next breakpoint
    pub fn continue_execution(&mut self) -> Result<()> {
        info!("Continuing execution...");
        self.paused = false;
        // TODO: Implement continue logic
        Ok(())
    }

    /// Pause execution at a function
    fn pause_at_function(&mut self, function: &str) {
        println!("\n[BREAKPOINT] Paused at function: {}", function);
        self.paused = true;
        self.state.set_current_function(function.to_string());
    }

    /// Check if debugger is currently paused
    pub fn is_paused(&self) -> bool {
        self.paused
    }

    /// Get current debug state
    pub fn state(&self) -> &DebugState {
        &self.state
    }

    /// Get mutable reference to breakpoint manager
    pub fn breakpoints_mut(&mut self) -> &mut BreakpointManager {
        &mut self.breakpoints
    }

    /// Get reference to executor
    pub fn executor(&self) -> &ContractExecutor {
        &self.executor
    }
}
