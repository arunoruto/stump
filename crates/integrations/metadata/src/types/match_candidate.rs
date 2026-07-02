use async_graphql::SimpleObject;
use serde::{Deserialize, Serialize};

use super::ExternalMetadata;

/// A potential match from an external provider
#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject)]
pub struct MatchCandidate {
	/// The provider this match came from
	pub provider: String,
	/// External ID on the provider's system
	pub external_id: String,
	pub metadata: ExternalMetadata,
	/// Confidence score (0.0 - 1.0)
	#[serde(default)]
	pub confidence: f32,
	/// Factors that contributed to the confidence score
	#[serde(default)]
	pub confidence_factors: Vec<ConfidenceFactor>,
}

/// A factor that contributed to a match's confidence score
#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject)]
pub struct ConfidenceFactor {
	/// Name of the scoring factor (e.g., "title_exact_match")
	pub factor: String,
	/// How much weight this factor carried
	pub weight: f32,
	/// Whether this factor matched
	pub matched: bool,
}

/// The result of a provider search: the candidates that were successfully
/// resolved, plus how many raw hits the search itself reported. Providers
/// like Hardcover require a separate per-hit detail fetch after the initial
/// search, and any of those can fail (e.g. rate limiting) without failing
/// the whole search -- `requested` lets callers detect when hits were
/// silently dropped instead of just seeing a shorter-than-expected list.
#[derive(Debug, Clone, Default)]
pub struct SearchOutcome {
	pub candidates: Vec<MatchCandidate>,
	pub requested: usize,
}

impl SearchOutcome {
	pub fn failed(&self) -> usize {
		self.requested.saturating_sub(self.candidates.len())
	}
}
