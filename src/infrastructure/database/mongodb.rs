use mongodb::{
    Client, Database,
    options::ClientOptions,
    bson::{doc, Document},
    error::Result,
};
use uuid::Uuid;
use crate::domain::{Galaxy, Star, Planet};

pub struct MongoDBConnection {
    database: Database,
}

impl MongoDBConnection {
    pub async fn new(connection_string: &str, database_name: &str) -> Result<Self> {
        let client_options = ClientOptions::parse(connection_string).await?;
        let client = Client::with_options(client_options)?;
        let database = client.database(database_name);
        
        Ok(Self { database })
    }

    pub async fn save_galaxy(&self, galaxy: &Galaxy) -> Result<()> {
        let collection = self.database.collection::<Document>("galaxies");
        let doc = doc! {
            "_id": galaxy.id.to_string(),
            "name": &galaxy.name,
            "seed": galaxy.seed,
            "size": {
                "radius": galaxy.size.radius,
                "thickness": galaxy.size.thickness,
            },
            "age": galaxy.age,
            "star_count": galaxy.star_count,
        };
        
        collection.insert_one(doc, None).await?;
        Ok(())
    }

    pub async fn save_star(&self, star: &Star) -> Result<()> {
        let collection = self.database.collection::<Document>("stars");
        let doc = doc! {
            "_id": star.id.to_string(),
            "galaxy_id": star.galaxy_id.to_string(),
            "name": &star.name,
            "position": {
                "x": star.position.x,
                "y": star.position.y,
                "z": star.position.z,
            },
            "mass": star.mass,
            "temperature": star.temperature,
            "age": star.age,
            "spectral_type": format!("{:?}", star.spectral_type),
        };
        
        collection.insert_one(doc, None).await?;
        Ok(())
    }

    pub async fn save_planet(&self, planet: &Planet) -> Result<()> {
        let collection = self.database.collection::<Document>("planets");
        let doc = doc! {
            "_id": planet.id.to_string(),
            "star_id": planet.star_id.to_string(),
            "name": &planet.name,
            "planet_type": format!("{:?}", planet.planet_type),
            "physical_properties": {
                "mass": planet.physical_properties.mass,
                "radius": planet.physical_properties.radius,
                "gravity": planet.physical_properties.gravity,
                "temperature": planet.physical_properties.temperature,
            },
        };
        
        collection.insert_one(doc, None).await?;
        Ok(())
    }

    pub async fn find_galaxy(&self, id: Uuid) -> Result<Option<Galaxy>> {
        let collection = self.database.collection::<Document>("galaxies");
        let filter = doc! { "_id": id.to_string() };
        
        if let Some(doc) = collection.find_one(filter, None).await? {
            // Document'i Galaxy'ye dönüştür
            // TODO: Implement conversion
            unimplemented!()
        } else {
            Ok(None)
        }
    }

    // Benzer şekilde find_star ve find_planet metodları...
} 