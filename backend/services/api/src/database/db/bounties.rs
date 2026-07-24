use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct BountyRequest {
    pub creator: String,
    pub title: String,
    pub description: String,
    pub budget: i128,
    pub deadline: u64,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct BountyApplication {
    pub bounty_id: u64,
    pub freelancer: String,
    pub proposal: String,
    pub proposed_budget: i128,
    pub timeline: u64,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Bounty {
    pub id: u64,
    pub title: String,
    pub description: String,
    pub budget: i128,
    pub deadline: u64,
    pub status: String,
    pub creator: String,
}

pub fn get_mock_bounties() -> Vec<Bounty> {
    vec![
        Bounty {
            id: 1,
            title: "Design a landing page".to_string(),
            description: "Create a modern, responsive landing page for a SaaS product".to_string(),
            budget: 5000,
            deadline: 1640995200, // 2021-12-31
            status: "open".to_string(),
            creator: "alex-studio".to_string(),
        },
        Bounty {
            id: 2,
            title: "Build API integration".to_string(),
            description: "Integrate third-party payment API into existing application".to_string(),
            budget: 3000,
            deadline: 1640995200,
            status: "in_progress".to_string(),
            creator: "jordan-dev".to_string(),
        },
    ]
}

pub fn get_bounty_by_id(bounty_id: u64) -> Option<Bounty> {
    let bounties = get_mock_bounties();
    bounties.into_iter().find(|bounty| bounty.id == bounty_id)
}

pub fn create_bounty(request: BountyRequest) -> Bounty {
    Bounty {
        id: 1, // In production, this would be generated
        title: request.title,
        description: request.description,
        budget: request.budget,
        deadline: request.deadline,
        status: "open".to_string(),
        creator: request.creator,
    }
}

pub fn apply_for_bounty(bounty_id: u64, application: BountyApplication) -> Result<(), String> {
    // In production, this would validate and store the application
    if application.proposal.trim().is_empty() {
        return Err("Proposal cannot be empty".to_string());
    }
    if application.proposed_budget <= 0 {
        return Err("Proposed budget must be positive".to_string());
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_application(bounty_id: u64) -> BountyApplication {
        BountyApplication {
            bounty_id,
            freelancer: "GFREELANCER".to_string(),
            proposal: "I can build this".to_string(),
            proposed_budget: 500,
            timeline: 7,
        }
    }

    #[test]
    fn get_bounty_by_id_returns_none_for_unknown_id() {
        assert!(get_bounty_by_id(9999).is_none());
    }

    #[test]
    fn get_bounty_by_id_returns_some_for_known_id() {
        let bounty = get_bounty_by_id(2).expect("bounty 2 exists in mock data");
        assert_eq!(bounty.creator, "jordan-dev");
    }

    #[test]
    fn create_bounty_always_starts_open() {
        let request = BountyRequest {
            creator: "test-creator".to_string(),
            title: "Test Bounty".to_string(),
            description: "Test Description".to_string(),
            budget: 1000,
            deadline: 1234567890,
        };
        let created = create_bounty(request);
        assert_eq!(created.status, "open");
        assert_eq!(created.budget, 1000);
    }

    #[test]
    fn apply_for_bounty_rejects_empty_proposal() {
        let mut application = sample_application(1);
        application.proposal = "   ".to_string();
        let result = apply_for_bounty(1, application);
        assert_eq!(result, Err("Proposal cannot be empty".to_string()));
    }

    #[test]
    fn apply_for_bounty_rejects_non_positive_budget() {
        let mut application = sample_application(1);
        application.proposed_budget = 0;
        let result = apply_for_bounty(1, application);
        assert_eq!(result, Err("Proposed budget must be positive".to_string()));

        let mut negative_application = sample_application(1);
        negative_application.proposed_budget = -100;
        let result = apply_for_bounty(1, negative_application);
        assert_eq!(result, Err("Proposed budget must be positive".to_string()));
    }

    #[test]
    fn apply_for_bounty_accepts_valid_application() {
        let application = sample_application(1);
        assert_eq!(apply_for_bounty(1, application), Ok(()));
    }
}
