use std::collections::HashMap;

/// Optimizes inventory levels based on demand forecasts and safety stock.
///
/// # Arguments
/// * `demand_forecast` - A hashmap of product IDs to their forecasted demand.
/// * `safety_stock` - A hashmap of product IDs to their safety stock levels.
///
/// # Returns
/// A hashmap of product IDs to optimal reorder levels.
pub fn optimize_inventory(
    demand_forecast: &HashMap<String, f64>,
    safety_stock: &HashMap<String, f64>,
) -> HashMap<String, f64> {
    let mut reorder_levels = HashMap::new();

    for (product, &forecast) in demand_forecast {
        let safety = safety_stock.get(product).unwrap_or(&0.0);
        reorder_levels.insert(product.clone(), forecast + safety);
    }

    reorder_levels
}

/// Finds the shortest route between locations using a naive approach.
///
/// # Arguments
/// * `distances` - A matrix representing distances between locations.
///
/// # Returns
/// A vector representing the order of visited locations.
pub fn optimize_route(distances: &[Vec<f64>]) -> Vec<usize> {
    let mut route = vec![];
    let mut visited = vec![false; distances.len()];

    let mut current_location = 0;
    route.push(current_location);
    visited[current_location] = true;

    while route.len() < distances.len() {
        let mut next_location = None;
        let mut min_distance = f64::INFINITY;

        for (i, &distance) in distances[current_location].iter().enumerate() {
            if !visited[i] && distance < min_distance {
                next_location = Some(i);
                min_distance = distance;
            }
        }

        if let Some(next) = next_location {
            route.push(next);
            visited[next] = true;
            current_location = next;
        } else {
            break;
        }
    }

    route
}
