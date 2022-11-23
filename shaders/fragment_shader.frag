#version 440
#define MAX_OBJECTS 128
out vec4 color;
in vec2 UV;

uniform float aspect_ratio;
uniform vec3 camera_position;
const vec3 light_dir = vec3(-1.0, 1.0, 0.0);
uniform float yaw;
uniform float pitch;
uniform float ambient_light;
uniform bool hard_shadows;
uniform float time;

struct Triangle {
    vec4 color;
    vec3 vertex0;
    vec3 vertex1;
    vec3 vertex2;
};

struct Hit {
    float t;
    vec3 point;
    vec3 normal;
    vec4 color;
    bool metal;
};

struct Ray {
    vec3 origin;
    vec3 direction;
};

vec3 reflect(vec3 v, vec3 n) {
    return v - 2.0 * dot(v,n)*n;
}

bool RayIntersectsTriangle(
    Triangle inTriangle,
    Ray ray,
    out Hit out_hit)
{
    const float EPSILON = 0.0000001;
    vec3 vertex0 = inTriangle.vertex0;
    vec3 vertex1 = inTriangle.vertex1;
    vec3 vertex2 = inTriangle.vertex2;
    vec3 edge1, edge2, h, s, q;
    float a,f,u,v;
    edge1 = vertex1 - vertex0;
    edge2 = vertex2 - vertex0;
    h = cross(ray.direction, edge2);// rayVector.crossProduct(edge2);
    a = dot(edge1, h);// edge1.dotProduct(h);
    if (a > -EPSILON && a < EPSILON)
        return false;    // This ray is parallel to this triangle.
    f = 1.0/a;
    s = ray.origin - vertex0;
    u = f * dot(s,h);// f * s.dotProduct(h);
    if (u < 0.0 || u > 1.0)
        return false;
    q = cross(s, edge1);// s.crossProduct(edge1);
    v = f * dot(ray.direction, q);// f * rayVector.dotProduct(q);
    if (v < 0.0 || u + v > 1.0)
        return false;
    // At this stage we can compute t to find out where the intersection point is on the line.
    float t = f * dot(edge2, q);//edge2.dotProduct(q);
    if (t > EPSILON) // ray intersection
    {
        out_hit.color = inTriangle.color;
        out_hit.t = t;

        vec3 normal = cross(edge2, edge1);
        out_hit.normal = (float(dot(normal, ray.direction) < 0.0) * 2.0 - 1.0) * normal;
        out_hit.point = ray.origin + ray.direction * t;
        return true;
    }
    else // This means that there is a line intersection but not a ray intersection -> triangle behind camera
        return false;
}

bool hit_spheres(Triangle[MAX_OBJECTS] triangles, Ray r, out Hit out_hit) {
    float smallest_t = 1000000.0;
    bool has_hit = false;
    Hit nearest_hit;// = Hit(0.0, vec3(0.0), vec3(0.0));
    for (int i = 0; i < triangles.length(); i++ ) {
        Hit h;

        if (RayIntersectsTriangle(triangles[i], r, h) && h.t < smallest_t) {
            has_hit = true;
            smallest_t = h.t;
            nearest_hit = h;

            out_hit = nearest_hit;

        }
    }

    return has_hit;
}

struct Sphere {
    float radius;
    vec3 position;
    vec4 color;
    bool metal;
};

uniform Sphere[MAX_OBJECTS] u_spheres;

bool hit_sphere(Sphere s, Ray r, out Hit returned_hit) {

    const float EPSILON = 0.0000001;

    if (s.radius <= EPSILON) {
        return false;
    }
    vec3 oc = r.origin - s.position;
    float a = dot(r.direction, r.direction);
    float b = 2.0 * dot(oc, r.direction);
    float c = dot(oc, oc) - s.radius * s.radius;
    float discriminant = b * b - 4.0 * a * c;
    if (discriminant >= EPSILON) {
        float t = (- b - sqrt(discriminant)) / (2.0 * a);
        if (t < EPSILON) {
            return false;
        }
        vec3 hit_point = r.direction * t + r.origin;
        vec3 normal = normalize((hit_point - s.position));
        if (s.color == vec4(-1)) {
            s.color = vec4(cos(time + 3.14 * 0.33 * 4) * 0.5 + 0.5, cos(time + 3.14 * 0.66) * 0.5 + 0.5, cos(time) * 0.5 + 0.5, 1.0);
        }
        returned_hit = Hit(t, hit_point, normal, s.color, s.metal);// s.color);
        return true;
    }

    return false;
}
bool hit_sphere_shadow(Sphere s, Ray r, out Hit returned_hit, out float depth) {

    const float EPSILON = 0.0000001;

    if (s.radius <= EPSILON) {
        return false;
    }
    vec3 oc = r.origin - s.position;
    float a = dot(r.direction, r.direction);
    float b = 2.0 * dot(oc, r.direction);
    float c = dot(oc, oc) - s.radius * s.radius;
    float discriminant = b * b - 4.0 * a * c;
    depth = discriminant;
    if (discriminant >= EPSILON) {
        float t = (- b - sqrt(discriminant)) / (2.0 * a);
        if (t < EPSILON) {
            return false;
        }
        vec3 hit_point = r.direction * t + r.origin;
        vec3 normal = normalize((hit_point - s.position));
        returned_hit = Hit(t, hit_point, normal, vec4(depth), s.metal);// s.color);

        return true;
    }

    return false;
}

bool hit_world(Sphere[MAX_OBJECTS] spheres, Ray r, out Hit out_hit) {
    float smallest_t = 1000000.0;
    bool has_hit = false;
    Hit nearest_hit;

    for (int i = 0; i < spheres.length(); i++ ) {
        Hit h;

        if (hit_sphere(spheres[i], r, h) && h.t < smallest_t) {
            has_hit = true;
            smallest_t = h.t;
            nearest_hit = h;

            out_hit = nearest_hit;

        }
    }
    
    return has_hit;
}
bool hit_world_shadow(Sphere[MAX_OBJECTS] spheres, Ray r, out Hit out_hit, out float depth) {
    float smallest_t = 1000000.0;
    bool has_hit = false;
    Hit nearest_hit;
    depth = 0.0f;
    for (int i = 0; i < spheres.length(); i++ ) {
        Hit h;
        float d;
        if (hit_sphere_shadow(spheres[i], r, h, d)) {
            if (h.t < smallest_t) {
                smallest_t = h.t;
                nearest_hit = h;
            }
            float d2 = sqrt(d) / (h.t);
            if (depth < d2) {
                depth = d2;
            }
            has_hit = true;


            out_hit = nearest_hit;
        }

    }
    return has_hit;
}
vec4 ray_color(Sphere[MAX_OBJECTS] spheres, Ray ray, int max_bounces) {

    float multiplier = 1.0;
    vec4 col = vec4(0.0);
    vec4 tint = vec4(1.0);
    Ray r = ray;


    for (int i = 0; i < max_bounces; i++) {

        float smallest_t = 10000000.0;

        Hit h;
        if (hit_world(spheres, r, h)) {
            float d;
            Hit _h;
            vec3 n_dir = normalize(light_dir);
            if (h.metal) {
                //Hit reflected_hit;

                tint *= h.color * multiplier;

                multiplier *= 0.5f;
                r.origin = h.point + h.normal * 0.001;
                r.direction = reflect(r.direction, h.normal);
                //if (hit_world(spheres, Ray(h.point, reflect(r.direction, h.normal)), reflected_hit)) {
                //    col += reflected_hit.color * max(dot(reflected_hit.normal, n_dir), 0.0) + vec4(ambient_light * reflected_hit.color.rgb, 0.0) * multiplier;
                //    //return col * h.color;
                //}
                //return col;

            }
            else {
                if (dot(h.normal, n_dir) < 0.0) {
                    col += (h.color * ambient_light) * multiplier;
                }

                else if (hit_world_shadow(spheres, Ray(h.point + h.normal * 0.001, light_dir), _h, d)) {
                    vec4 soft_shadow = vec4(0.0);
                    if (!hard_shadows) {
                        soft_shadow = max(h.color * max(dot(h.normal, n_dir), 0.0) * min(-sqrt(d * 2) + 1.25, 1.0), vec4(0.0));
                    }



                    col += (h.color * ambient_light + soft_shadow) * multiplier;
                    //return vec4(d);
                    //col.b = 0.0;

                }
                else {
                    col += (h.color * max(dot(h.normal, n_dir), 0.0) + vec4(ambient_light * h.color.rgb, 0.0)) * multiplier;
                }
                return col * tint;
            }
        }
        else {
            return (col + vec4(0.1, 0.2, 0.6, 1.0)) * multiplier * tint;
        }
    }
    return col * tint;
}



void main() {


    vec3 ray_dir = normalize(vec3((UV.x - 0.5) * aspect_ratio, UV.y -0.5, - 0.5 ));
    vec3 rotated_ray_dir_pitch = vec3(ray_dir.x, ray_dir.y * cos(pitch) - ray_dir.z * sin(pitch), cos(pitch) * ray_dir.z + sin(pitch) * ray_dir.y);
    vec3 rotated_ray_dir = vec3(cos(yaw) * rotated_ray_dir_pitch.x - sin(yaw) * rotated_ray_dir_pitch.z, rotated_ray_dir_pitch.y, cos(yaw) * rotated_ray_dir_pitch.z + sin(yaw) * rotated_ray_dir_pitch.x);

    Ray r = Ray(camera_position, rotated_ray_dir);
    color = ray_color(u_spheres, r, 5);

}
